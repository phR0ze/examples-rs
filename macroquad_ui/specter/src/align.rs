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

    /// Calculate the position vector based on the given widget size and positioning directive as well
    /// as the containing widget's size and optional position.
    /// * `size` is the size of the widget to position
    /// * `cont_size` is the containing widget's size
    /// * `cont_pos` is the containing positional to offset
    ///
    /// ### Examples
    /// ```
    /// use specter::prelude::*;
    ///
    /// let size = vec2(2., 2.);
    /// let cont_size = vec2(4., 4.);
    /// let cont_pos = vec2(2., 2.);
    /// assert_eq!(Align::LeftTop.relative(size, cont_size, cont_pos), vec2(2., 2.));
    /// ```
    pub fn relative(&self, size: Vec2, cont_size: Vec2, cont_pos: Vec2) -> Vec2 {
        let pos = match self {
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
        pos + cont_pos
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

    use super::*;
    #[test]
    fn relative() {
        let (w, h) = (450., 800.);
        let size = vec2(20., 20.);
        let cpos = vec2(10., 10.);
        let csize = vec2(w, h);

        assert_eq!(Align::CenterTop.relative(size, csize, cpos), vec2((w - size.x) / 2., 0.) + cpos);
        assert_eq!(Align::Center.relative(size, csize, cpos), vec2((w - size.x) / 2., (h - size.y) / 2.) + cpos);
        assert_eq!(Align::CenterBottom.relative(size, csize, cpos), vec2((w - size.x) / 2., h - size.y) + cpos);
        assert_eq!(Align::RightTop.relative(size, csize, cpos), vec2(w - size.x, 0.) + cpos);
        assert_eq!(Align::RightCenter.relative(size, csize, cpos), vec2(w - size.x, (h - size.y) / 2.) + cpos);
        assert_eq!(Align::RightBottom.relative(size, csize, cpos), vec2(w - size.x, h - size.y) + cpos);
        assert_eq!(Align::LeftTop.relative(size, csize, cpos), vec2(0., 0.) + cpos);
        assert_eq!(Align::LeftCenter.relative(size, csize, cpos), vec2(0., (h - size.y) / 2.) + cpos);
        assert_eq!(Align::LeftBottom.relative(size, csize, cpos), vec2(0., h - size.y) + cpos);
        assert_eq!(Align::Static(10., 10.).relative(size, csize, cpos), vec2(10., 10.) + cpos);
    }
}
