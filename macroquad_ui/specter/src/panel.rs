//! Panel provides a container widget with options for framing and layout
use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct PanelBuilder {
    frame: Frame,   // frame properties
    layout: Layout, // layout properties
}

impl PanelBuilder {
    pub fn new() -> Self {
        Self {
            frame: Frame::new(),
            layout: Layout::new(""),
        }
    }

    /// Set the frame's properties
    pub fn frame(self, f: impl FnOnce(Frame) -> Frame) -> Self {
        Self {
            frame: f(self.frame),
            ..self
        }
    }

    /// Set layout to use
    pub fn layout(self, f: impl FnOnce(Layout) -> Layout) -> Self {
        Self {
            layout: f(self.layout),
            ..self
        }
    }

    /// Create a new Panel instance
    pub fn build<T: AsRef<str>>(&self, id: T) -> Panel {
        Panel {
            frame: self.frame.clone(),
            layout: self.layout.clone().id(id.as_ref()),
            widgets: vec![],
        }
    }
}

pub struct Panel {
    frame: Frame,                  // frame properties
    layout: Layout,                // layout properties
    widgets: Vec<Box<dyn Widget>>, // widgets to draw
}

// Constructors and builders
impl Panel {
    pub fn new<T: AsRef<str>>(id: T) -> Self {
        Self {
            frame: Frame::new(),
            layout: Layout::new(id),
            widgets: vec![],
        }
    }

    /// Create a horizontal panel
    /// * lays out sub-layouts using the left to right packing mode
    pub fn horz<T: AsRef<str>>(id: T) -> Self {
        Self::new(id).layout(|x| x.mode(Mode::LeftToRight))
    }

    /// Create a vertical layout
    /// * lays out sub-layouts using the top to bottom packing mode
    pub fn vert<T: AsRef<str>>(id: T) -> Self {
        Self::new(id).layout(|x| x.mode(Mode::TopToBottom))
    }

    /// Set the frame's properties
    pub fn frame(self, f: impl FnOnce(Frame) -> Frame) -> Self {
        Self {
            frame: f(self.frame),
            ..self
        }
    }

    /// Set layout to use
    pub fn layout(self, f: impl FnOnce(Layout) -> Layout) -> Self {
        Self {
            layout: f(self.layout),
            ..self
        }
    }
}

// Utility functions
impl Panel {
    /// Draw the widget on the screen
    /// * `ui` is the Macroquad Ui engine
    fn ui(&mut self, ui: &mut Ui) -> Response {
        let mut context = ui.get_active_window_context();

        // Get panel position and size
        let (pos, size) = self.layout.shape();

        // Register click intention for the panel
        let rect = Rect::new(pos.x, pos.y, size.x as f32, size.y as f32);
        let (hovered, clicked) = context.register_click_intention(rect);

        // Draw background based on click intention
        if hovered && self.frame.image_hov.is_some() {
            let image = self.frame.image_hov.unwrap();
            widgets::Texture::new(image).size(size.x, size.y).position(pos).ui(ui);
        } else if clicked && self.frame.image_clk.is_some() {
            let image = self.frame.image_clk.unwrap();
            widgets::Texture::new(image).size(size.x, size.y).position(pos).ui(ui);
        } else if let Some(image) = self.frame.image {
            widgets::Texture::new(image).size(size.x, size.y).position(pos).ui(ui);
        } else if let Some(color) = self.frame.fill {
            draw_rectangle(pos.x, pos.y, size.x, size.y, color);
        }

        // Draw widgets
        for x in self.widgets.iter_mut() {
            x.show_p(ui);
        }

        Response { clicked, hovered }
    }
}

impl LayoutManager for Panel {
    /// Add the given widget to this widget's layout management
    /// * similar to `append` but consumes and returns self
    fn add(mut self, widget: impl Widget + 'static) -> Self {
        self.append(widget);
        self
    }

    /// Add the given widget to this widget's layout management
    fn append(&mut self, widget: impl Widget + 'static) {
        self.layout.append(&widget.layout_ref());
        self.widgets.push(Box::new(widget));
    }
}

impl Widget for Panel {
    /// Returns a reference clone to the Widget's layout
    fn layout_ref(&self) -> Layout {
        self.layout.ptr()
    }

    /// Draw the widget on the screen
    /// * `ui` is the Macroquad Ui engine
    fn show_p(&mut self, ui: &mut Ui) -> Response {
        self.ui(ui)
    }
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_foo() {
        assert_eq!(vec2(2., 2.), vec2(2., 2.));
    }
}
