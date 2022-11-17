//! Frame provides properties for controlling how a widget's frame appears
//! * background properties
//! * stroke properties
use crate::prelude::*;

/// Frame is not a widget or container but rather provides a set of properties for manipulating a
/// widget's or container's
/// * background properties
/// * stroke properties
#[derive(Clone, Debug)]
pub struct Frame {
    pub interact: bool,               // tracks if an interactive feature was configured
    pub fill: Option<Color>,          // fill color to use
    pub fill_clk: Option<Color>,      // fill color to use when clicked
    pub fill_hov: Option<Color>,      // fill color to use when hovered
    pub image: Option<Texture2D>,     // background image to use
    pub image_clk: Option<Texture2D>, // background image to use when clicked
    pub image_hov: Option<Texture2D>, // background image to use when hovered
}

// Constructors and builders
impl Frame {
    pub fn new() -> Self {
        Frame {
            interact: false,
            fill: Some(GRAY),
            fill_clk: None,
            fill_hov: None,
            image: None,
            image_clk: None,
            image_hov: None,
        }
    }

    /// Set the fill color
    pub fn fill(self, color: Color) -> Self {
        Self {
            fill: Some(color),
            ..self
        }
    }

    /// Set the fill color when clicked
    pub fn fill_clk(self, color: Color) -> Self {
        Self {
            interact: true,
            fill_clk: Some(color),
            ..self
        }
    }

    /// Set the fill color when hovered
    pub fn fill_hov(self, color: Color) -> Self {
        Self {
            interact: true,
            fill_hov: Some(color),
            ..self
        }
    }

    /// Set background image to use
    pub fn image<T: Into<Texture2D>>(self, image: T) -> Self {
        Self {
            image: Some(image.into()),
            ..self
        }
    }

    /// Set background image to use
    pub fn image_clk<T: Into<Texture2D>>(self, image: T) -> Self {
        Self {
            interact: true,
            image_clk: Some(image.into()),
            ..self
        }
    }

    /// Set background image to use
    pub fn image_hov<T: Into<Texture2D>>(self, image: T) -> Self {
        Self {
            interact: true,
            image_hov: Some(image.into()),
            ..self
        }
    }
}

// Utility functions
impl Frame {
    /// Get the fill color
    pub fn get_fill(&self) -> Option<Color> {
        self.fill
    }

    /// Set the fill color
    pub fn set_fill(&mut self, color: Color) -> &mut Self {
        self.fill = Some(color);
        self
    }
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

    //use super::*;

    #[test]
    fn test() {
        //
    }
}
