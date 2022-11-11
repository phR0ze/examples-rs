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
    ///
    /// ### Examples
    /// ```
    /// use specter::prelude::*;
    ///
    /// assert_eq!(Align::Center.is_static(), false);
    /// assert_eq!(Align::Static(0., 0.).is_static(), true);
    /// ```
    pub fn is_static(&self) -> bool {
        match self {
            Align::Static(_, _) => true,
            _ => false,
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
        Align::Static(val.x, val.y)
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
