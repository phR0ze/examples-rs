use crate::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Panel {
    size: Vec2,
    color: Color,
}

impl Panel {
    pub fn new(color: Color) -> Self {
        Self { size: vec2(50., 50.), color }
    }

    /// Draw the widget on the screen
    /// * `layout` provides layout directive support for
    pub fn show<F: FnOnce(&mut Ui, &mut Layout)>(&mut self, ui: &mut Ui, layout: &mut Layout, f: F) {
        let (pos, size) = layout.alloc(self.size);
        draw_rectangle(pos.x, pos.y, size.x, size.y, self.color);
        f(ui, layout)
    }
}
