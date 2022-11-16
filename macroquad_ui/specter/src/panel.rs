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

// Getters
impl Panel {
    /// Get the frame's properties
    pub fn get_frame(&self) -> &Frame {
        &self.frame
    }
}

// Setters
impl Panel {
    /// Set the frame's properties
    pub fn set_frame(&mut self, f: impl FnOnce(Frame) -> Frame) {
        self.frame = f(self.frame.clone());
    }
}

// Utility functions
impl LayoutManager for Panel {
    /// Adds the widget to this widget's layout management
    /// * `widget` is the widget being added
    fn append(&mut self, widget: impl Widget + 'static) {
        self.layout.subs_append(&widget.layout_ref());
        self.widgets.push(Box::new(widget));
    }
}

impl Widget for Panel {
    /// Get the widget's layout as a cloned reference
    fn layout_ref(&self) -> Layout {
        self.layout.ptr()
    }

    /// Draw the widget on the screen
    /// * `ui` is the Macroquad Ui engine
    fn show(&mut self, ui: &mut Ui) {
        // Draw panel
        let (pos, size) = self.layout.shape();
        draw_rectangle(pos.x, pos.y, size.x, size.y, self.frame.fill);

        // Draw widgets
        for x in self.widgets.iter_mut() {
            x.show(ui);
        }
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
