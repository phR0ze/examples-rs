//! Frame provides properties for controlling how a widget's frame appears
//! * background properties
//! * stroke properties
use crate::prelude::*;

/// Frame is not a widget or container but rather provides a set of properties for manipulating a
/// widget's or container's
/// * background properties
/// * stroke properties
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Frame {
    //pub rounding: Rounding,
    //pub shadow: Shadow,
    /// Color to fill the frame with
    pub fill: Color,
    //pub stroke: Stroke,
}

// Constructors and builders
impl Frame {
    pub fn new() -> Self {
        Frame { fill: GRAY }
    }

    /// Set the fill color
    pub fn with_fill(self, color: Color) -> Self {
        Self {
            fill: color,
            ..self
        }
    }
}

// Getters and setters
impl Frame {
    /// Get the fill color
    pub fn fill(&self) -> Color {
        self.fill
    }

    /// Set the fill color
    pub fn set_fill(&mut self, color: Color) -> &mut Self {
        self.fill = color;
        self
    }
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_frame_setters() {
        let mut frame = Frame::new();
        assert_eq!(vec2(2., 2.), vec2(2., 2.));
    }
}
