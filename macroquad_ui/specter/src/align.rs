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
    /// * is removed from normal padding and margins rules providing absolute positioning
    Absolute(f32, f32),
}

impl Align {
    /// Returns true if we have an absolute alignment
    ///
    /// ### Examples
    /// ```
    /// use specter::prelude::*;
    ///
    /// assert_eq!(Align::Center.is_static(), false);
    /// assert_eq!(Align::Static(0., 0.).is_static(), true);
    /// ```
    pub fn is_absolute(&self) -> bool {
        match self {
            Align::Absolute(_, _) => true,
            _ => false,
        }
    }

    /// Calculates the layout's position relative to its parent layout
    /// * `layout` is the layout's size
    /// * `parent` is the layout's parenent's size
    ///
    /// ### Examples
    /// ```
    /// use specter::prelude::*;
    ///
    /// assert_eq!(Align::Center.relative(vec2(20., 20.), vec2(100., 100.)), vec2(40., 40.));
    /// ```
    pub fn relative(&self, layout: Vec2, parent: Vec2) -> Vec2 {
        match self {
            Align::CenterTop => vec2((parent.x - layout.x) / 2.0, 0.0),
            Align::Center => vec2(parent.x - layout.x, parent.y - layout.y) / 2.0,
            Align::CenterBottom => vec2((parent.x - layout.x) / 2.0, parent.y - layout.y),
            Align::RightTop => vec2(parent.x - layout.x, 0.0),
            Align::RightCenter => vec2(parent.x - layout.x, (parent.y - layout.y) / 2.0),
            Align::RightBottom => vec2(parent.x - layout.x, parent.y - layout.y),
            Align::LeftTop => vec2(0.0, 0.0),
            Align::LeftCenter => vec2(0.0, (parent.y - layout.y) / 2.0),
            Align::LeftBottom => vec2(0.0, parent.y - layout.y),
            Align::Absolute(x, y) => vec2(*x, *y),
        }
    }
}

impl Default for Align {
    /// Create the default Alignment directive a.k.a LeftTop
    ///
    /// ### Examples
    /// ```
    /// use specter::prelude::*;
    ///
    /// assert_eq!(Align::default(), Align::LeftTop);
    /// ```
    fn default() -> Self {
        Align::LeftTop
    }
}

impl From<Vec2> for Align {
    /// Create the default Alignment directive a.k.a LeftTop
    ///
    /// ### Examples
    /// ```
    /// use specter::prelude::*;
    ///
    /// assert_eq!(Align::from(vec2(1., 2.)), Align::Static(1., 2.));
    /// ```
    fn from(val: Vec2) -> Self {
        Align::Absolute(val.x, val.y)
    }
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

    //use super::*;
    #[test]
    fn relative() {
        //
    }
}
