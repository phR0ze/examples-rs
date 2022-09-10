//! Group provides a relatively positioned MQ group that will adjust to maintain its
//! original position relative to the app window size. This is a work around for the
//! stock MQ window that has a static position regardless of application window size
//! changes.
//!
//! Inspiration for this work around comes from https://github.com/fishfolks/jumpy
use crate::position::Position;
use macroquad::{
    prelude::*,
    ui::{root_ui, widgets, Id, Skin, Ui},
};

const NO_COLOR: Color = Color::new(0.0, 0.0, 0.0, 0.0);

pub struct Group {
    id: Id,
    size: Vec2,
    skin: Skin,
    padding: RectOffset,
    position: Position,
}

impl Group {
    /// Create a new group instance.
    /// WARNING: Call this outside the main ui loop to avoid odd ui behavior
    pub fn new(id: Id, size: Vec2, background: Image) -> Self {
        // Configure skin for relative positioning work around.
        // Keeping this as an internal implementation detail
        let skin = {
            // NO_COLOR gets rid of the weird 1px white border around the group
            let group_style = root_ui().style_builder().color(NO_COLOR).build();
            let button_style = root_ui().style_builder().background(background.clone()).build();
            Skin { group_style, button_style, ..root_ui().default_skin() }
        };

        Group { id, size, skin, padding: RectOffset::new(0., 0., 0., 0.), position: Position::default() }
    }

    /// Content is indented by the given rectangle from the edges.
    /// This works even when size() is used unlike stock MQ styles.
    #[allow(dead_code)]
    pub fn padding(self, padding: RectOffset) -> Self {
        Group { padding, ..self }
    }

    /// Return the size of the group
    #[allow(dead_code)]
    pub fn size(&self) -> Vec2 {
        self.size
    }

    /// Configure initial position of the group on the screen
    #[allow(dead_code)]
    pub fn position<T: Into<Position>>(&mut self, pos: T) -> &Self {
        self.position = pos.into();
        self
    }

    /// Draw the group. The callback `f` will be called with the current `Ui` instance and
    /// the available content size of the group as arguments
    pub fn ui<F: FnOnce(&mut Ui, Vec2)>(&self, ui: &mut Ui, f: F) {
        ui.push_skin(&self.skin);

        // Calculate desired position
        let bg_position = match self.position {
            Position::Center => vec2(screen_width() - self.size.x, screen_height() - self.size.y) / 2.0,
            Position::Absolute(position) => position,
        };

        // Draw solid blue rectangle the same size as the group as first layer.
        // This is useful for understanding padding and sizing while designing
        //draw_rectangle(bg_position.x, bg_position.y, self.size.x, self.size.y, BLUE);

        // Draw button as workaround for background image
        widgets::Button::new("").size(self.size).position(bg_position).ui(ui);

        // Calculate group size and position taking padding into account.
        // Padding reduces the group size and shifts position to even it out.
        let group_size = vec2(
            self.size.x - self.padding.left - self.padding.right,
            self.size.y - self.padding.top - self.padding.bottom,
        );
        let group_position = vec2(bg_position.x + self.padding.left, bg_position.y + self.padding.top);
        //draw_rectangle(group_position.x, group_position.y, group_size.x, group_size.y, GREEN);

        // Group provides a box to layout out any widgets inside that overlays
        // the non-interactive button.
        widgets::Group::new(self.id, group_size).position(group_position).ui(ui, |ui| {
            ui.pop_skin();
            f(ui, group_size)
        });

        // Together they form window like functionality that can resize dynamnically
        // based on the application window size changes. MQ's stock window doesn't
        // provide this ability; instead it is static regardless of parent window size
    }
}
