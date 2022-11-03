use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Panel {
    id: String,  // panel identifier
    dirty: bool, // track if the widget needs styling and shape calculation updates

    // Configuration
    frame: Frame,   // panel frame properties
    layout: Layout, // layout
}

impl Panel {
    pub fn new<T: AsRef<str>>(id: T) -> Self {
        Self {
            id: id.as_ref().to_string(),
            dirty: true,
            layout: Layout::vert(id.as_ref()),
            frame: Frame::default(),
        }
    }

    /// Set the fill color
    pub fn fill(self, color: Color) -> Self {
        Self { frame: self.frame.fill(color), ..self }
    }

    /// Set layout to use
    pub fn layout<F: FnOnce(Layout) -> Layout>(self, f: F) -> Self {
        Self { layout: f(self.layout), ..self }
    }

    /// Allocate space inside this widget for the given widget
    pub fn alloc(&self, widget: &impl Widget) {
        self.layout.append(&widget.layout_ref())
    }

    /// Draw the widget on the screen
    /// * `layout` parent layout to draw button within
    /// * returns true when clicked in the current frame
    pub fn show<F: FnOnce(&mut Ui, &Layout)>(&mut self, ui: &mut Ui, layout: Option<&Layout>, f: F) {
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
