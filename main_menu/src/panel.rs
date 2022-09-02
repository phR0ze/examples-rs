//! Panel provides a relatively positioned MQ window that will adjust to maintain its
//! original position relative to the app window size. This is a work around for the
//! stock MQ window that has a static position regardless of application window size
//! changes.
//!
//! Inspiration for this work around comes from https://github.com/fishfolks/jumpy
use macroquad::{
    prelude::*,
    ui::{widgets, Id, Skin, Ui},
};

const NO_COLOR: Color = Color::new(0.0, 0.0, 0.0, 0.0);

pub struct Panel {
    id: Id,
    size: Vec2,
    skin: Skin,
    position: Vec2,
}

impl Panel {
    // Create a new panel instance.
    // WARNING: Call this outside the main ui loop to avoid odd ui behavior
    pub fn new(ui: &mut Ui, id: Id, size: Vec2, panel_bg: Image) -> Self {
        // Configure panel skin for work around solution
        let skin = {
            let group_style = ui
                .style_builder()
                .color(NO_COLOR)
                .color_hovered(NO_COLOR)
                .color_clicked(NO_COLOR)
                .build();
            let button_style = ui
                .style_builder()
                .background(panel_bg.clone())
                .background_hovered(panel_bg.clone())
                .background_clicked(panel_bg)
                .background_margin(RectOffset::new(52.0, 52.0, 52.0, 52.0))
                .build();
            Skin {
                group_style,
                button_style,
                ..ui.default_skin()
            }
        };

        Panel {
            id,
            size,
            skin,
            position: vec2(0., 0.),
        }
    }

    // Center the position of the panel on the app window
    #[allow(dead_code)]
    pub fn center(&mut self) -> &Self {
        self.position = vec2(
            screen_width() / 2.0 - self.size.x / 2.0,
            screen_height() / 2.0 - self.size.y / 2.0,
        );
        self
    }

    // Set the position of the panel
    #[allow(dead_code)]
    pub fn position(&mut self, pos: Vec2) -> &Self {
        self.position = pos;
        self
    }

    /// Draw the panel. The callback `f` will be called with the current `Ui` instance and
    /// the available content size of the panel as arguments
    pub fn ui<F: FnOnce(&mut Ui, Vec2)>(&self, ui: &mut Ui, f: F) {
        ui.push_skin(&self.skin);

        // Non-interactive button provides ability to draw the panel background
        let _ = widgets::Button::new("")
            .position(self.position)
            .size(self.size)
            .ui(ui);

        // // TEST
        // draw_rectangle(
        //     self.position.x + Self::BG_OFFSET,
        //     self.position.y + Self::BG_OFFSET,
        //     self.size.x - (Self::BG_OFFSET * 2.0),
        //     self.size.y - (Self::BG_OFFSET * 2.0),
        //     background_color,
        // );

        // Group provides a box to layout out any widgets inside that overlays
        // the non-interactive button.
        widgets::Group::new(self.id, self.size)
            .position(self.position)
            .ui(ui, |ui| {
                ui.pop_skin();
                f(ui, self.size)
            });

        // Together they form window like functionality that can resize dynamnically
        // based on the application window size changes. MQ's stock window doesn't
        // provide this ability; instead it is static regardless of parent window size
    }
}
