//! Align provides functionality for calculating positioning for widgets in their parent layouts
//!
use crate::prelude::*;

/// Align is a directive used to guide the calculation of the widgets position in its parent layout
#[derive(Clone, Debug, PartialEq)]
pub enum Align {
    /// Align widget in the center horizontally and in the top vertically
    /// * accepts an optional offset value
    CenterTop(Option<RectOffset>),

    /// Align in the center horizontally and in the center vertically
    /// * accepts an optional offset value
    Center(Option<RectOffset>),

    /// Align in the center horizontally and in the bottom vertically
    /// * accepts an optional offset value
    CenterBottom(Option<RectOffset>),

    /// Align in the right horizontally and in the top vertically
    /// * accepts an optional offset value
    RightTop(Option<RectOffset>),

    /// Align in the right horizontally and in the center vertically
    /// * accepts an optional offset value
    RightCenter(Option<RectOffset>),

    /// Align in the right horizontally and in the bottom vertically
    /// * accepts an optional offset value
    RightBottom(Option<RectOffset>),

    /// Align in the left horizontally and in the top vertically
    /// * accepts an optional offset value
    LeftTop(Option<RectOffset>),

    /// Align in the left horizontally and in the center vertically
    /// * accepts an optional offset value
    LeftCenter(Option<RectOffset>),

    /// Align in the left horizontally and in the bottom vertically
    /// * accepts an optional offset value
    LeftBottom(Option<RectOffset>),

    /// Align horizontally with the given value and vertically with the given value
    Static(f32, f32),
}

impl Align {
    /// Returns true if we have a static alignment
    pub fn is_static(&self) -> bool {
        match self {
            Align::Static(_, _) => true,
            _ => false,
        }
    }

    /// Calculate the position vector based on the given widget size and positioning directive as well
    /// as the containing widget's size and optional position.
    /// * `size` is the size of the widget to position
    /// * `cont_size` is the containing widget's size
    /// * `cont_pos` is the containing positional to offset
    pub fn relative(&self, size: Vec2, cont_size: Vec2, cont_pos: Option<Vec2>) -> Vec2 {
        let mut pos = match self {
            Align::CenterTop(None) => vec2((cont_size.x - size.x) / 2.0, 0.0),
            Align::CenterTop(Some(offset)) => {
                vec2((cont_size.x - size.x) / 2.0 + offset.left - offset.right, offset.top - offset.bottom)
            },
            Align::Center(None) => vec2(cont_size.x - size.x, cont_size.y - size.y) / 2.0,
            Align::Center(Some(offset)) => vec2(
                (cont_size.x - size.x) / 2.0 + offset.left - offset.right,
                (cont_size.y - size.y) / 2.0 + offset.top - offset.bottom,
            ),
            Align::CenterBottom(None) => vec2((cont_size.x - size.x) / 2.0, cont_size.y - size.y),
            Align::CenterBottom(Some(offset)) => vec2(
                (cont_size.x - size.x) / 2.0 + offset.left - offset.right,
                cont_size.y - size.y + offset.top - offset.bottom,
            ),
            Align::RightTop(None) => vec2(cont_size.x - size.x, 0.0),
            Align::RightTop(Some(offset)) => {
                vec2(cont_size.x - size.x + offset.left - offset.right, offset.top - offset.bottom)
            },
            Align::RightCenter(None) => vec2(cont_size.x - size.x, (cont_size.y - size.y) / 2.0),
            Align::RightCenter(Some(offset)) => vec2(
                cont_size.x - size.x + offset.left - offset.right,
                (cont_size.y - size.y) / 2.0 + offset.top - offset.bottom,
            ),
            Align::RightBottom(None) => vec2(cont_size.x - size.x, cont_size.y - size.y),
            Align::RightBottom(Some(offset)) => vec2(
                cont_size.x - size.x + offset.left - offset.right,
                cont_size.y - size.y - offset.top - offset.bottom,
            ),
            Align::LeftTop(None) => vec2(0.0, 0.0),
            Align::LeftTop(Some(offset)) => vec2(offset.left - offset.right, offset.top - offset.bottom),
            Align::LeftCenter(None) => vec2(0.0, (cont_size.y - size.y) / 2.0),
            Align::LeftCenter(Some(offset)) => {
                vec2(offset.left - offset.right, (cont_size.y - size.y) / 2.0 + offset.top - offset.bottom)
            },
            Align::LeftBottom(None) => vec2(0.0, cont_size.y - size.y),
            Align::LeftBottom(Some(offset)) => {
                vec2(offset.left - offset.right, cont_size.y - size.y + offset.top - offset.bottom)
            },
            Align::Static(x, y) => vec2(*x, *y),
        };

        // If the containing widget's position was given offset by that amount
        if let Some(start) = cont_pos {
            pos.x += start.x;
            pos.y += start.y;
        }
        pos
    }

    /// Scale the positional values for mobile use
    pub fn scale(&self) -> Align {
        match self {
            Align::CenterTop(None) => Align::CenterTop(None),
            Align::CenterTop(Some(offset)) => Align::CenterTop(Some(scale_rect_p(*offset))),
            Align::Center(None) => Align::Center(None),
            Align::Center(Some(offset)) => Align::Center(Some(scale_rect_p(*offset))),
            Align::CenterBottom(None) => Align::CenterBottom(None),
            Align::CenterBottom(Some(offset)) => Align::CenterBottom(Some(scale_rect_p(*offset))),
            Align::RightTop(None) => Align::RightTop(None),
            Align::RightTop(Some(offset)) => Align::RightTop(Some(scale_rect_p(*offset))),
            Align::RightCenter(None) => Align::RightCenter(None),
            Align::RightCenter(Some(offset)) => Align::RightCenter(Some(scale_rect_p(*offset))),
            Align::RightBottom(None) => Align::RightBottom(None),
            Align::RightBottom(Some(offset)) => Align::RightBottom(Some(scale_rect_p(*offset))),
            Align::LeftTop(None) => Align::LeftTop(None),
            Align::LeftTop(Some(offset)) => Align::LeftTop(Some(scale_rect_p(*offset))),
            Align::LeftCenter(None) => Align::LeftCenter(None),
            Align::LeftCenter(Some(offset)) => Align::LeftCenter(Some(scale_rect_p(*offset))),
            Align::LeftBottom(None) => Align::LeftBottom(None),
            Align::LeftBottom(Some(offset)) => Align::LeftBottom(Some(scale_rect_p(*offset))),
            Align::Static(x, y) => Align::Static(*x, *y),
        }
    }

    /// Calculate the position vector based on the given component size
    /// * `target` is the component's size to be taken into account
    pub fn vec2(&self, target: Vec2) -> Vec2 {
        let container = screen();
        match self {
            Align::CenterTop(x) => Align::CenterTop(*x).relative(target, container, None),
            Align::Center(x) => Align::Center(*x).relative(target, container, None),
            Align::CenterBottom(x) => Align::CenterBottom(*x).relative(target, container, None),
            Align::RightTop(x) => Align::RightTop(*x).relative(target, container, None),
            Align::RightCenter(x) => Align::RightCenter(*x).relative(target, container, None),
            Align::RightBottom(x) => Align::RightBottom(*x).relative(target, container, None),
            Align::LeftTop(x) => Align::LeftTop(*x).relative(target, container, None),
            Align::LeftCenter(x) => Align::LeftCenter(*x).relative(target, container, None),
            Align::LeftBottom(x) => Align::LeftBottom(*x).relative(target, container, None),
            Align::Static(x, y) => vec2(*x, *y),
        }
    }
}

impl Default for Align {
    fn default() -> Self {
        Align::Center(None)
    }
}

impl From<Vec2> for Align {
    fn from(val: Vec2) -> Self {
        Align::Static(val.x, val.y)
    }
}
