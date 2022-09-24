//! Position provides additional functionality around positioning widgets
use macroquad::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum Position {
    Center,
    TopCenter,
    TopRight(Option<RectOffset>), // optional margin to allow around the position
    TopLeft(Option<RectOffset>),  // optional margin to allow around the position
    Absolute(Vec2),
}

impl Position {
    /// Calculate the position vector based on the given component size
    /// * `size` is the component's size to be taken into account
    pub fn vec2(&self, size: Vec2) -> Vec2 {
        match self {
            Position::Center => vec2(screen_width() - size.x, screen_height() - size.y) / 2.0,
            Position::TopCenter => vec2(screen_width() - size.x, 0.0) / 2.0,
            Position::TopRight(Some(margin)) => vec2(screen_width() - size.x - margin.right, margin.top),
            Position::TopRight(None) => vec2(screen_width() - size.x, 0.0),
            Position::TopLeft(Some(margin)) => vec2(margin.left, margin.top),
            Position::TopLeft(None) => vec2(0.0, 0.0),
            Position::Absolute(position) => *position,
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position::Center
    }
}

impl From<Vec2> for Position {
    fn from(position: Vec2) -> Self {
        Self::Absolute(position)
    }
}
