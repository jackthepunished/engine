//! UI system for 2D interface elements
//!
//! Provides widgets, layout, and event handling.

mod rect;
mod widget;

pub use rect::{Anchor, Rect, RectStyle};
pub use widget::{Button, Label, Panel, Widget, WidgetState};
