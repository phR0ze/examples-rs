use crate::prelude::*;

/// Frame is not a widget or container but rather provides a set of properties for manipulating a
/// widget's or container's
/// * margin and padding
/// * background color or image
/// * stroke properties
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Frame {
    /// Provides space outside the frame
    pub margin: RectOffset,
    /// Provides space inside the frame
    pub padding: RectOffset,
    //pub rounding: Rounding,
    //pub shadow: Shadow,
    /// Color to fill the frame with
    pub fill: Color,
    //pub stroke: Stroke,
}
