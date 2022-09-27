//! Position provides additional functionality around positioning widgets
use macroquad::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum Position {
    Center,
    TopCenter,

    /// Position in the top right of the containing group
    /// * Optional margin to allow around the position
    TopRight(Option<RectOffset>),

    /// Position in the top left of the containing group
    /// * Optional margin to allow around the position
    TopLeft(Option<RectOffset>),

    Custom(Vec2),
}

impl Position {
    /// Position of origin
    pub fn origin() -> Vec2 {
        vec2(0.0, 0.0)
    }

    /// Calculate the position vector based on the given containing group
    /// * `size` is the size of the target component to position
    /// * `group_size` is the containing group's size
    /// * `group_pos` is the containing group's position
    pub fn relative(&self, size: Vec2, group_size: Vec2, group_pos: Vec2) -> Vec2 {
        match self {
            Position::Center => {
                vec2(group_pos.x + group_size.x - size.x, group_pos.y + group_size.y - size.y) / 2.0
            },
            Position::TopCenter => vec2(0.0, 0.0),
            Position::TopRight(Some(margin)) => vec2(0.0, 0.0),
            Position::TopRight(None) => vec2(0.0, 0.0),
            Position::TopLeft(Some(margin)) => vec2(0.0, 0.0),
            Position::TopLeft(None) => vec2(0.0, 0.0),
            Position::Custom(position) => *position,
        }
    }

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
            Position::Custom(position) => *position,
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
        Self::Custom(position)
    }
}
