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
        }
    }
}

#[derive(Debug, Clone)]
pub struct Panel {
    frame: Frame,   // frame properties
    layout: Layout, // layout properties
}

// Constructors and builders
impl Panel {
    pub fn new<T: AsRef<str>>(id: T) -> Self {
        Self {
            frame: Frame::new(),
            layout: Layout::new(id),
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
impl Panel {
    /// Draw the widget on the screen
    /// * `ui` is the Macroquad Ui engine
    /// * returns true when clicked in the current frame
    pub fn show(&mut self, ui: &mut Ui) {
        self.show_pf(ui, None, |_, _| {})
    }

    /// Draw the widget on the screen
    /// * `ui` is the Macroquad Ui engine
    /// * `f` is a lambda for child layout creation
    /// * returns true when clicked in the current frame
    pub fn show_f(&mut self, ui: &mut Ui, f: impl FnOnce(&mut Ui, &Layout)) {
        self.show_pf(ui, None, f)
    }

    /// Draw the widget on the screen
    /// * `ui` is the Macroquad Ui engine
    /// * `layout` parent layout to draw button within
    /// * returns true when clicked in the current frame
    pub fn show_p(&mut self, ui: &mut Ui, layout: &Layout) {
        self.show_pf(ui, Some(layout), |_, _| {})
    }

    /// Draw the widget on the screen
    /// * `ui` is the Macroquad Ui engine
    /// * `layout` parent layout to draw button within
    /// * `f` is a lambda for child layout creation
    /// * returns true when clicked in the current frame
    pub fn show_pf(&mut self, ui: &mut Ui, layout: Option<&Layout>, f: impl FnOnce(&mut Ui, &Layout)) {
        if let Some(parent) = layout {
            parent.subs_append(&self.layout);
        }

        // Draw panel
        let (pos, size) = self.layout.shape();
        draw_rectangle(pos.x, pos.y, size.x, size.y, self.frame.fill);

        // Draw widgets
        f(ui, &self.layout)
    }
}

impl Widget for Panel {
    /// Get the widget's layout as a cloned reference
    fn layout_g(&self) -> Layout {
        self.layout.layout_g()
    }
}

impl Widget for &Panel {
    /// Get the widget's layout as a cloned reference
    fn layout_g(&self) -> Layout {
        self.layout.layout_g()
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
