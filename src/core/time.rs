//! Time management for the game loop

use std::time::{Duration, Instant};

/// Tracks time between frames and total elapsed time
#[derive(Debug)]
pub struct Time {
    /// Time since engine started
    start_time: Instant,
    /// Time of last frame
    last_frame: Instant,
    /// Duration of last frame (delta time)
    delta: Duration,
    /// Total elapsed time since start
    elapsed: Duration,
    /// Frame count
    frame_count: u64,
}

impl Time {
    /// Create a new Time tracker
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            last_frame: now,
            delta: Duration::ZERO,
            elapsed: Duration::ZERO,
            frame_count: 0,
        }
    }

    /// Update time at the start of each frame
    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_frame;
        self.last_frame = now;
        self.elapsed = now - self.start_time;
        self.frame_count += 1;
    }

    /// Get delta time in seconds
    pub fn delta_seconds(&self) -> f32 {
        self.delta.as_secs_f32()
    }

    /// Get delta time as Duration
    pub fn delta(&self) -> Duration {
        self.delta
    }

    /// Get total elapsed time in seconds
    pub fn elapsed_seconds(&self) -> f32 {
        self.elapsed.as_secs_f32()
    }

    /// Get total elapsed time
    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }

    /// Get the current frame count
    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    /// Get current FPS (averaged over last frame)
    pub fn fps(&self) -> f32 {
        if self.delta.as_secs_f32() > 0.0 {
            1.0 / self.delta.as_secs_f32()
        } else {
            0.0
        }
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::new()
    }
}
