use crate::prelude::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Layout {
    widgets: Vec<(Vec2, Color)>,
    x: f32,
    y: f32,
}

impl Layout {
    pub fn new() -> Self {
        Layout::default()
    }

    /// Add the given widget to the layout
    pub fn add(&mut self, widget: Vec2, color: Color) -> &mut Self {
        self.widgets.push((widget, color));
        self
    }

    /// Show the various widgets using the layout's policy for positioning
    pub fn show<F: FnOnce(&mut Ui)>(&mut self, ui: &mut Ui, f: F) {
        for x in self.widgets.iter() {
            draw_rectangle(self.x, 0., x.0.x, x.0.y, x.1);

            // Horizontal
            self.x += x.0.x;
        }
    }
}
