//! Position provides functionality for positioning widgets on the screen
use crate::prelude::*;

/// Position is a directive used to guide the calculation of the actual position
#[derive(Debug, Copy, Clone)]
pub enum Position {
    /// Position in the center horizontally and in the top vertically
    /// * accepts and optional offset value
    CenterTop(Option<RectOffset>),

    /// Position in the center horizontally and in the center vertically
    /// * accepts and optional offset value
    Center(Option<RectOffset>),

    /// Position in the center horizontally and in the bottom vertically
    /// * accepts and optional offset value
    CenterBottom(Option<RectOffset>),

    /// Position in the right horizontally and in the center vertically
    /// * accepts and optional offset value
    RightCenter(Option<RectOffset>),

    /// Position in the right horizontally and in the top vertically
    /// * accepts and optional offset value
    RightTop(Option<RectOffset>),

    /// Position in the left horizontally and in the top vertically
    /// * accepts and optional offset value
    LeftTop(Option<RectOffset>),

    /// Position in the left horizontally and in the center vertically
    /// * accepts and optional offset value
    LeftCenter(Option<RectOffset>),

    /// Position in the left horizontally and in the bottom vertically
    /// * accepts and optional offset value
    LeftBottom(Option<RectOffset>),

    /// Position horizontally with the given value and vertically with the given value
    Static(f32, f32),
}

impl Position {
    /// Returns the origin position
    pub fn origin() -> Vec2 {
        vec2(0.0, 0.0)
    }

    /// Scale the positional margins
    pub fn scale(&self) -> Position {
        match self {
            Position::CenterTop(None) => Position::CenterTop(None),
            Position::CenterTop(Some(offset)) => Position::CenterTop(Some(scale_rect_p(*offset))),
            Position::Center(None) => Position::Center(None),
            Position::Center(Some(offset)) => Position::Center(Some(scale_rect_p(*offset))),
            Position::CenterBottom(None) => Position::CenterBottom(None),
            Position::CenterBottom(Some(offset)) => Position::CenterBottom(Some(scale_rect_p(*offset))),
            Position::RightCenter(None) => Position::RightCenter(None),
            Position::RightCenter(Some(offset)) => Position::RightCenter(Some(scale_rect_p(*offset))),
            Position::RightTop(None) => Position::RightTop(None),
            Position::RightTop(Some(offset)) => Position::RightTop(Some(scale_rect_p(*offset))),
            Position::LeftTop(None) => Position::LeftTop(None),
            Position::LeftTop(Some(offset)) => Position::LeftTop(Some(scale_rect_p(*offset))),
            Position::LeftCenter(None) => Position::LeftCenter(None),
            Position::LeftCenter(Some(offset)) => Position::LeftCenter(Some(scale_rect_p(*offset))),
            Position::LeftBottom(None) => Position::LeftBottom(None),
            Position::LeftBottom(Some(offset)) => Position::LeftBottom(Some(scale_rect_p(*offset))),
            Position::Static(x, y) => Position::Static(*x, *y),
        }
    }

    /// Calculate the position vector based on the given widget size and positioning directive as well
    /// as the containing widget's size and optionally the containing widget's position.
    /// * `size` is the size of the target component to position
    /// * `cont_size` is the containing widget's size
    /// * `cont_pos_offset` is the containing positioning offset to account for
    pub fn relative(&self, size: Vec2, cont_size: Vec2, cont_pos_offset: Option<Vec2>) -> Vec2 {
        let mut pos = match self {
            Position::CenterTop(None) => vec2((cont_size.x - size.x) / 2.0, 0.0),
            Position::CenterTop(Some(offset)) => {
                vec2((cont_size.x - size.x) / 2.0 + offset.left - offset.right, offset.top - offset.bottom)
            },
            Position::Center(None) => vec2(cont_size.x - size.x, cont_size.y - size.y) / 2.0,
            Position::Center(Some(offset)) => vec2(
                (cont_size.x - size.x) / 2.0 + offset.left - offset.right,
                (cont_size.y - size.y) / 2.0 + offset.top - offset.bottom,
            ),
            Position::CenterBottom(None) => vec2((cont_size.x - size.x) / 2.0, cont_size.y - size.y),
            Position::CenterBottom(Some(offset)) => vec2(
                (cont_size.x - size.x) / 2.0 + offset.left - offset.right,
                cont_size.y - size.y + offset.top - offset.bottom,
            ),
            Position::RightCenter(None) => vec2(cont_size.x - size.x, (cont_size.y - size.y) / 2.0),
            Position::RightCenter(Some(offset)) => vec2(
                cont_size.x - size.x + offset.left - offset.right,
                (cont_size.y - size.y) / 2.0 + offset.top - offset.bottom,
            ),
            Position::RightTop(None) => vec2(cont_size.x - size.x, 0.0),
            Position::RightTop(Some(offset)) => {
                vec2(cont_size.x - size.x + offset.left - offset.right, offset.top - offset.bottom)
            },
            Position::LeftTop(None) => vec2(0.0, 0.0),
            Position::LeftTop(Some(offset)) => vec2(offset.left - offset.right, offset.top - offset.bottom),
            Position::LeftCenter(None) => vec2(0.0, (cont_size.y - size.y) / 2.0),
            Position::LeftCenter(Some(offset)) => {
                vec2(offset.left - offset.right, (cont_size.y - size.y) / 2.0 + offset.top - offset.bottom)
            },
            Position::LeftBottom(None) => vec2(0.0, cont_size.y - size.y),
            Position::LeftBottom(Some(offset)) => {
                vec2(offset.left - offset.right, cont_size.y - size.y + offset.top - offset.bottom)
            },
            Position::Static(x, y) => vec2(*x, *y),
        };
        if let Some(start) = cont_pos_offset {
            pos.x += start.x;
            pos.y += start.y;
        }
        pos
    }

    /// Calculate the position vector based on the given component size
    /// * `target` is the component's size to be taken into account
    pub fn vec2(&self, target: Vec2) -> Vec2 {
        let container = Size::screen();
        match self {
            Position::CenterTop(x) => Position::CenterTop(*x).relative(target, container, None),
            Position::Center(x) => Position::Center(*x).relative(target, container, None),
            Position::CenterBottom(x) => Position::CenterBottom(*x).relative(target, container, None),
            Position::RightCenter(x) => Position::RightCenter(*x).relative(target, container, None),
            Position::RightTop(x) => Position::RightTop(*x).relative(target, container, None),
            Position::LeftTop(x) => Position::LeftTop(*x).relative(target, container, None),
            Position::LeftCenter(x) => Position::LeftCenter(*x).relative(target, container, None),
            Position::LeftBottom(x) => Position::LeftBottom(*x).relative(target, container, None),
            Position::Static(x, y) => vec2(*x, *y),
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position::Center(None)
    }
}
