use crate::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Panel {
    id: String,
    size: Vec2,
    color: Color,
}

impl Panel {
    pub fn new<T: AsRef<str>>(id: T, color: Color) -> Self {
        Self { id: id.as_ref().to_string(), size: vec2(50., 50.), color }
    }

    /// Draw the widget on the screen
    /// * `layout` provides layout directive support for
    pub fn show<F: FnOnce(&mut Ui, &mut Layout)>(&mut self, ui: &mut Ui, layout: &mut Layout, f: F) {
        let (pos, size) = layout.alloc(&self.id, self.size);
        draw_rectangle(pos.x, pos.y, size.x, size.y, self.color);
        f(ui, layout)
    }
}
