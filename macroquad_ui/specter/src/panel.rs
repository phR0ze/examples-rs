//! Panel provides a container widget with options for framing and layout
use crate::prelude::*;

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
            layout: Layout::horz(id),
        }
    }

    /// Set the frame's properties
    pub fn with_frame(self, f: impl FnOnce(Frame) -> Frame) -> Self {
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

// Getters and setters
impl Panel {
    /// Get the frame's properties
    pub fn frame(&self) -> Frame {
        self.frame
    }

    /// Set the frame's properties
    pub fn set_frame(&mut self, f: impl FnOnce(Frame) -> Frame) -> &mut Self {
        self.frame = f(self.frame);
        self
    }

    /// Draw the widget on the screen
    /// * `layout` parent layout to draw button within
    /// * returns true when clicked in the current frame
    pub fn show(&mut self, ui: &mut Ui, layout: Option<&Layout>, f: impl FnOnce(&mut Ui, &Layout)) {
        if let Some(parent) = layout {
            parent.append(&self.layout);
        }

        // Draw panel
        let (pos, size) = self.layout.shape();
        draw_rectangle(pos.x, pos.y, size.x, size.y, self.frame.fill);

        // Draw widgets
        f(ui, &self.layout)
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
