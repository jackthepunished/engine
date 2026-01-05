//! UI widgets
//!
//! Provides interactive UI elements.

use glam::Vec2;

use super::rect::Rect;

/// Widget state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WidgetState {
    /// Normal state
    #[default]
    Normal,
    /// Mouse hovering
    Hovered,
    /// Being pressed
    Pressed,
    /// Disabled
    Disabled,
}

/// Trait for UI widgets
pub trait Widget {
    /// Get the rectangle bounds
    fn rect(&self) -> &Rect;

    /// Get mutable rectangle
    fn rect_mut(&mut self) -> &mut Rect;

    /// Get current state
    fn state(&self) -> WidgetState;

    /// Handle mouse movement
    fn on_mouse_move(&mut self, position: Vec2, parent_size: Vec2);

    /// Handle mouse button down
    fn on_mouse_down(&mut self, position: Vec2, parent_size: Vec2) -> bool;

    /// Handle mouse button up
    fn on_mouse_up(&mut self, position: Vec2, parent_size: Vec2) -> bool;
}

/// A clickable button
#[derive(Debug, Clone)]
pub struct Button {
    /// Rectangle
    pub rect: Rect,
    /// Label text
    pub text: String,
    /// Current state
    state: WidgetState,
    /// Whether button was clicked this frame
    clicked: bool,
}

impl Button {
    /// Create a new button
    #[must_use]
    pub fn new(text: impl Into<String>, rect: Rect) -> Self {
        Self {
            rect,
            text: text.into(),
            state: WidgetState::Normal,
            clicked: false,
        }
    }

    /// Check if button was clicked (resets after check)
    pub fn was_clicked(&mut self) -> bool {
        let result = self.clicked;
        self.clicked = false;
        result
    }

    /// Disable or enable the button
    pub fn set_disabled(&mut self, disabled: bool) {
        self.state = if disabled {
            WidgetState::Disabled
        } else {
            WidgetState::Normal
        };
    }

    /// Check if button is disabled
    #[must_use]
    pub fn is_disabled(&self) -> bool {
        self.state == WidgetState::Disabled
    }
}

impl Widget for Button {
    fn rect(&self) -> &Rect {
        &self.rect
    }

    fn rect_mut(&mut self) -> &mut Rect {
        &mut self.rect
    }

    fn state(&self) -> WidgetState {
        self.state
    }

    fn on_mouse_move(&mut self, position: Vec2, parent_size: Vec2) {
        if self.state == WidgetState::Disabled {
            return;
        }

        if self.rect.contains(position, parent_size) {
            if self.state != WidgetState::Pressed {
                self.state = WidgetState::Hovered;
            }
        } else if self.state != WidgetState::Pressed {
            self.state = WidgetState::Normal;
        }
    }

    fn on_mouse_down(&mut self, position: Vec2, parent_size: Vec2) -> bool {
        if self.state == WidgetState::Disabled {
            return false;
        }

        if self.rect.contains(position, parent_size) {
            self.state = WidgetState::Pressed;
            true
        } else {
            false
        }
    }

    fn on_mouse_up(&mut self, position: Vec2, parent_size: Vec2) -> bool {
        if self.state == WidgetState::Pressed {
            if self.rect.contains(position, parent_size) {
                self.clicked = true;
                self.state = WidgetState::Hovered;
                return true;
            }
            self.state = WidgetState::Normal;
        }
        false
    }
}

/// A text label
#[derive(Debug, Clone)]
pub struct Label {
    /// Rectangle
    pub rect: Rect,
    /// Text content
    pub text: String,
    /// Text color (RGBA)
    pub color: [f32; 4],
}

impl Label {
    /// Create a new label
    #[must_use]
    pub fn new(text: impl Into<String>, rect: Rect) -> Self {
        Self {
            rect,
            text: text.into(),
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }

    /// Set text color
    #[must_use]
    pub fn with_color(mut self, color: [f32; 4]) -> Self {
        self.color = color;
        self
    }
}

impl Widget for Label {
    fn rect(&self) -> &Rect {
        &self.rect
    }

    fn rect_mut(&mut self) -> &mut Rect {
        &mut self.rect
    }

    fn state(&self) -> WidgetState {
        WidgetState::Normal
    }

    fn on_mouse_move(&mut self, _position: Vec2, _parent_size: Vec2) {}
    fn on_mouse_down(&mut self, _position: Vec2, _parent_size: Vec2) -> bool {
        false
    }
    fn on_mouse_up(&mut self, _position: Vec2, _parent_size: Vec2) -> bool {
        false
    }
}

/// A container panel
#[derive(Debug, Clone)]
pub struct Panel {
    /// Rectangle
    pub rect: Rect,
    /// Panel title (optional)
    pub title: Option<String>,
}

impl Panel {
    /// Create a new panel
    #[must_use]
    pub fn new(rect: Rect) -> Self {
        Self { rect, title: None }
    }

    /// Set title
    #[must_use]
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl Widget for Panel {
    fn rect(&self) -> &Rect {
        &self.rect
    }

    fn rect_mut(&mut self) -> &mut Rect {
        &mut self.rect
    }

    fn state(&self) -> WidgetState {
        WidgetState::Normal
    }

    fn on_mouse_move(&mut self, _position: Vec2, _parent_size: Vec2) {}
    fn on_mouse_down(&mut self, _position: Vec2, _parent_size: Vec2) -> bool {
        false
    }
    fn on_mouse_up(&mut self, _position: Vec2, _parent_size: Vec2) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_click() {
        let rect = Rect::new(10.0, 10.0, 100.0, 30.0);
        let mut button = Button::new("Test", rect);
        let parent = Vec2::new(800.0, 600.0);
        let inside = Vec2::new(50.0, 25.0);

        // Simulate click
        button.on_mouse_down(inside, parent);
        assert_eq!(button.state(), WidgetState::Pressed);

        button.on_mouse_up(inside, parent);
        assert!(button.was_clicked());
        assert_eq!(button.state(), WidgetState::Hovered);
    }

    #[test]
    fn test_button_click_outside() {
        let rect = Rect::new(10.0, 10.0, 100.0, 30.0);
        let mut button = Button::new("Test", rect);
        let parent = Vec2::new(800.0, 600.0);
        let outside = Vec2::new(200.0, 200.0);

        button.on_mouse_down(outside, parent);
        assert_eq!(button.state(), WidgetState::Normal);
    }
}
