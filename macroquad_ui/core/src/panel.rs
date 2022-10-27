use crate::prelude::*;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Panel {
    id: String,   // panel identifier
    size: Vec2,   //
    frame: Frame, // panel frame properties
}

impl Panel {
    pub fn new<T: AsRef<str>>(id: T) -> Self {
        Self { id: id.as_ref().to_string(), size: vec2(50., 50.), frame: Frame::default() }
    }

    /// Set the fill color
    pub fn with_fill(self, color: Color) -> Self {
        Self { frame: Frame { fill: color, ..self.frame }, ..self }
    }

    /// Draw the widget on the screen
    /// * `layout` provides layout directive support for
    pub fn show<F: FnOnce(&mut Ui, &mut Layout)>(&mut self, ui: &mut Ui, layout: &mut Layout, f: F) {
        // let (pos, size) = layout.alloc(&self.id, self.size);
        // draw_rectangle(pos.x, pos.y, size.x, size.y, self.frame.fill);
        // f(ui, layout)
    }
}
