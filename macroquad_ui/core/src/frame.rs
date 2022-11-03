use crate::prelude::*;

/// Frame is not a widget or container but rather provides a set of properties for manipulating a
/// widget's or container's
/// * background color or image
/// * stroke properties
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Frame {
    //pub rounding: Rounding,
    //pub shadow: Shadow,
    /// Color to fill the frame with
    pub fill: Color,
    //pub stroke: Stroke,
}

impl Frame {
    /// Set the fill color
    pub fn fill(self, color: Color) -> Self {
        Self { fill: color, ..self }
    }
}
