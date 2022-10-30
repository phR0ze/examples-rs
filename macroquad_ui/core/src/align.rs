//! Align provides functionality for calculating positioning for widgets in their parent layouts
//!
use crate::prelude::*;

/// Align is a directive used to guide the calculation of the widgets position in its parent layout
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Align {
    /// Align widget in the center horizontally and in the top vertically
    CenterTop,

    /// Align in the center horizontally and in the center vertically
    Center,

    /// Align in the center horizontally and in the bottom vertically
    CenterBottom,

    /// Align in the right horizontally and in the top vertically
    RightTop,

    /// Align in the right horizontally and in the center vertically
    RightCenter,

    /// Align in the right horizontally and in the bottom vertically
    RightBottom,

    /// Align in the left horizontally and in the top vertically
    LeftTop,

    /// Align in the left horizontally and in the center vertically
    LeftCenter,

    /// Align in the left horizontally and in the bottom vertically
    LeftBottom,

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
    pub fn relative(&self, size: Vec2, cont_size: Vec2, cont_pos: Vec2) -> Vec2 {
        let mut pos = match self {
            Align::CenterTop => vec2((cont_size.x - size.x) / 2.0, 0.0),
            Align::Center => vec2(cont_size.x - size.x, cont_size.y - size.y) / 2.0,
            Align::CenterBottom => vec2((cont_size.x - size.x) / 2.0, cont_size.y - size.y),
            Align::RightTop => vec2(cont_size.x - size.x, 0.0),
            Align::RightCenter => vec2(cont_size.x - size.x, (cont_size.y - size.y) / 2.0),
            Align::RightBottom => vec2(cont_size.x - size.x, cont_size.y - size.y),
            Align::LeftTop => vec2(0.0, 0.0),
            Align::LeftCenter => vec2(0.0, (cont_size.y - size.y) / 2.0),
            Align::LeftBottom => vec2(0.0, cont_size.y - size.y),
            Align::Static(x, y) => vec2(*x, *y),
        };

        // If the containing widget's position was given offset by that amount
        pos.x += cont_pos.x;
        pos.y += cont_pos.y;
        pos
    }
}

impl Default for Align {
    fn default() -> Self {
        Align::LeftTop
    }
}

impl From<Vec2> for Align {
    fn from(val: Vec2) -> Self {
        Align::Static(val.x, val.y)
    }
}
