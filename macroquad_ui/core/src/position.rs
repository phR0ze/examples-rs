//! Position provides additional functionality around positioning widgets
use crate::prelude::*;

/// Position is a directive used to calculate the actual position. The enum values are
/// named in a WidthHeight camel case describing the width and height positioning directive.
#[derive(Debug, Copy, Clone)]
pub enum Position {
    /// Position center horizontally and center vertically
    /// * accepts and optional offset value
    Center(Option<RectOffset>),

    /// Position center horizontally and top vertically
    /// * accepts and optional offset value
    CenterTop(Option<RectOffset>),

    /// Position on the right horizontally and center vertically
    /// * accepts and optional offset value
    RightCenter(Option<RectOffset>),

    /// Position on the right horizontally and top vertically
    /// * accepts and optional offset value
    RightTop(Option<RectOffset>),

    /// Position on the left horizontally and center vertically
    /// * accepts and optional offset value
    LeftCenter(Option<RectOffset>),

    /// Position on the left horizontally and top vertically
    /// * accepts and optional offset value
    LeftTop(Option<RectOffset>),

    /// Custom position relative to the containing widget
    Custom(f32, f32),
}

impl Position {
    /// Position of origin
    pub fn origin() -> Vec2 {
        vec2(0.0, 0.0)
    }

    /// Scale the positional margins
    pub fn scale(&self) -> Position {
        match self {
            Position::Center(None) => Position::Center(None),
            Position::Center(Some(offset)) => Position::Center(Some(scale_rect_p(*offset))),
            Position::CenterTop(None) => Position::CenterTop(None),
            Position::CenterTop(Some(offset)) => Position::CenterTop(Some(scale_rect_p(*offset))),
            Position::RightCenter(None) => Position::RightCenter(None),
            Position::RightCenter(Some(offset)) => Position::RightCenter(Some(scale_rect_p(*offset))),
            Position::RightTop(None) => Position::RightTop(None),
            Position::RightTop(Some(offset)) => Position::RightTop(Some(scale_rect_p(*offset))),
            Position::LeftCenter(None) => Position::LeftCenter(None),
            Position::LeftCenter(Some(offset)) => Position::LeftCenter(Some(scale_rect_p(*offset))),
            Position::LeftTop(None) => Position::LeftTop(None),
            Position::LeftTop(Some(offset)) => Position::LeftTop(Some(scale_rect_p(*offset))),
            Position::Custom(x, y) => Position::Custom(*x, *y),
        }
    }

    /// Calculate the position vector based on the given widget size and the
    /// containing widget size and the positioning directive.
    /// * `target` is the size of the target component to position
    /// * `container` is the containing widget's size
    /// * `start` when given is an optional offset to account for containers that don't reset position to (0,0)
    pub fn relative(&self, target: Vec2, container: Vec2, start: Option<Vec2>) -> Vec2 {
        let mut pos = match self {
            Position::Center(None) => vec2(container.x - target.x, container.y - target.y) / 2.0,
            Position::Center(Some(offset)) => vec2(
                (container.x - target.x) / 2.0 + offset.left - offset.right,
                (container.y - target.y) / 2.0 + offset.top - offset.bottom,
            ),
            Position::CenterTop(None) => vec2((container.x - target.x) / 2.0, 0.0),
            Position::CenterTop(Some(offset)) => {
                vec2((container.x - target.x) / 2.0 + offset.left - offset.right, offset.top - offset.bottom)
            },
            Position::RightCenter(None) => vec2(container.x - target.x, (container.y - target.y) / 2.0),
            Position::RightCenter(Some(offset)) => vec2(
                container.x - target.x + offset.left - offset.right,
                (container.y - target.y) / 2.0 + offset.top - offset.bottom,
            ),
            Position::RightTop(None) => vec2(container.x - target.x, 0.0),
            Position::RightTop(Some(offset)) => {
                vec2(container.x - target.x + offset.left - offset.right, offset.top - offset.bottom)
            },
            Position::LeftCenter(None) => vec2(0.0, (container.y - target.y) / 2.0),
            Position::LeftCenter(Some(offset)) => {
                vec2(offset.left - offset.right, (container.y - target.y) / 2.0 + offset.top - offset.bottom)
            },
            Position::LeftTop(None) => vec2(0.0, 0.0),
            Position::LeftTop(Some(offset)) => vec2(offset.left - offset.right, offset.top - offset.bottom),
            Position::Custom(x, y) => vec2(*x, *y),
        };
        if let Some(start) = start {
            pos.x += start.x;
            pos.y += start.y;
        }
        pos
    }

    /// Calculate the position vector based on the given component size
    /// * `target` is the component's size to be taken into account
    pub fn vec2(&self, target: Vec2) -> Vec2 {
        let container = vec2(screen_width(), screen_height());
        match self {
            Position::Center(x) => Position::Center(*x).relative(target, container, None),
            Position::CenterTop(x) => Position::CenterTop(*x).relative(target, container, None),
            Position::RightCenter(x) => Position::RightCenter(*x).relative(target, container, None),
            Position::RightTop(x) => Position::RightTop(*x).relative(target, container, None),
            Position::LeftCenter(x) => Position::LeftCenter(*x).relative(target, container, None),
            Position::LeftTop(x) => Position::LeftTop(*x).relative(target, container, None),
            Position::Custom(x, y) => vec2(*x, *y),
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position::Center(None)
    }
}
