use crate::prelude::*;

pub struct ImageBuilder {
    base: PanelBuilder,
}

impl ImageBuilder {
    pub fn new() -> Self {
        Self {
            base: PanelBuilder::new(),
        }
    }

    /// Set the frame's properties
    pub fn frame(self, f: impl FnOnce(Frame) -> Frame) -> Self {
        Self {
            base: self.base.frame(f),
        }
    }

    /// Enable listening for click interactions
    pub fn interact(self) -> Self {
        Self {
            base: self.base.interact(),
        }
    }

    /// Disable listening for click interactions
    pub fn no_interact(self) -> Self {
        Self {
            base: self.base.no_interact(),
        }
    }

    /// Set layout to use
    pub fn layout(self, f: impl FnOnce(Layout) -> Layout) -> Self {
        Self {
            base: self.base.layout(f),
        }
    }

    /// Create a new Image instance
    pub fn build<T: AsRef<str>, U: Into<Texture2D>>(&self, id: T, image: U) -> Image {
        Image {
            base: self.base.build(id).frame(|x| x.image(image)),
        }
    }
}

pub struct Image {
    base: Panel,
}

impl Image {
    pub fn new<T: AsRef<str>, U: Into<Texture2D>>(id: T, image: U) -> Image {
        ImageBuilder::new().build(id, image)
    }

    /// Set the frame's properties
    pub fn frame(self, f: impl FnOnce(Frame) -> Frame) -> Self {
        Self {
            base: self.base.frame(f),
        }
    }

    /// Set image to use
    pub fn image<T: Into<Texture2D>>(self, image: T) -> Self {
        Self {
            base: self.base.frame(|x| x.image(image)),
        }
    }

    /// Set image to use
    pub fn image_clk<T: Into<Texture2D>>(self, image: T) -> Self {
        Self {
            base: self.base.frame(|x| x.image_clk(image)),
        }
    }

    /// Set image to use
    pub fn image_hov<T: Into<Texture2D>>(self, image: T) -> Self {
        Self {
            base: self.base.frame(|x| x.image_hov(image)),
        }
    }

    /// Enable listening for click interactions
    pub fn interact(self) -> Self {
        Self {
            base: self.base.interact(),
        }
    }

    /// Disable listening for click interactions
    pub fn no_interact(self) -> Self {
        Self {
            base: self.base.no_interact(),
        }
    }

    /// Set layout to use
    pub fn layout(self, f: impl FnOnce(Layout) -> Layout) -> Self {
        Self {
            base: self.base.layout(f),
        }
    }
}

impl LayoutManager for Image {
    /// Add the given widget to this widget's layout management
    /// * similar to `append` but consumes and returns self
    fn add(mut self, widget: impl Widget + 'static) -> Self {
        self.append(widget);
        self
    }

    /// Add the given widget to this widget's layout management
    fn append(&mut self, widget: impl Widget + 'static) {
        self.base.append(widget);
    }
}

impl Widget for Image {
    /// Returns a reference clone to the Widget's layout
    fn layout_ptr(&self) -> Layout {
        self.base.layout_ptr()
    }

    /// Draw the widget on the screen
    /// * `ui` is the Macroquad Ui engine
    fn show_p(&mut self, ui: &mut Ui) -> Response {
        self.base.show_p(ui)
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
