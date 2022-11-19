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

/// Default implementation for Frame
impl Default for Frame {
    fn default() -> Self {
        Frame {
            interact: false,
            fill: None,
            fill_clk: None,
            fill_hov: None,
            image: None,
            image_clk: None,
            image_hov: None,
        }
    }
}

// Constructors and builders
impl Frame {
    pub fn new() -> Self {
        Self::default().fill(GRAY)
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

    /// Remove the fill color
    pub fn no_fill(self) -> Self {
        Self { fill: None, ..self }
    }

    /// Remove the fill color when clicked
    pub fn no_fill_clk(self) -> Self {
        Self {
            interact: self.image_clk.is_some() || self.image_hov.is_some() || self.fill_hov.is_some(),
            fill_clk: None,
            ..self
        }
    }

    /// Remove the fill color when hovered
    pub fn no_fill_hov(self) -> Self {
        Self {
            interact: self.image_clk.is_some() || self.image_hov.is_some() || self.fill_clk.is_some(),
            fill_hov: None,
            ..self
        }
    }

    /// Remove background image to use
    pub fn no_image(self) -> Self {
        Self {
            image: None,
            ..self
        }
    }

    /// Remove background image to use
    pub fn no_image_clk(self) -> Self {
        Self {
            interact: self.image_hov.is_some() || self.fill_hov.is_some() || self.fill_clk.is_some(),
            image_clk: None,
            ..self
        }
    }

    /// Remove background image to use
    pub fn no_image_hov(self) -> Self {
        Self {
            interact: self.image_clk.is_some() || self.fill_hov.is_some() || self.fill_clk.is_some(),
            image_hov: None,
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

    use super::*;

    #[test]
    fn interact() {
        let icon = Texture2D::from_file_with_format(include_bytes!("../assets/options_icon.png"), None);

        // Enabling
        // assert_eq!(Frame::new().interact, false);
        // assert_eq!(Frame::new().fill_clk(BLACK).interact, true);
        // assert_eq!(Frame::new().fill_hov(BLACK).interact, true);
        // assert_eq!(Frame::new().image_clk(icon.clone()).interact, true);
        // assert_eq!(Frame::new().image_hov(icon.clone()).interact, true);
    }
}
