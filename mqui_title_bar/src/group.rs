//! Group provides a relatively positioned MQ group that will adjust to maintain its
//! original position relative to the app window size. This is a work around for the
//! stock MQ window that has a static position regardless of application window size
//! changes.
//!
//! Inspiration for this work-around comes from https://github.com/fishfolks/jumpy
use crate::position::Position;
use macroquad::{
    prelude::*,
    ui::{root_ui, widgets, Id, Skin, Ui},
};

const NO_COLOR: Color = Color::new(0.0, 0.0, 0.0, 0.0);

#[derive(Debug, Clone)]
pub struct GroupStyle {
    pub padding: RectOffset,         // padding to apply around title
    pub background: Option<Image>,   // optional background image to use
    pub border_color: Option<Color>, // optional border color to use
}

impl GroupStyle {
    pub fn new() -> Self {
        GroupStyle { padding: RectOffset::new(0., 0., 0., 0.), background: None, border_color: None }
    }

    /// Inset the content by the given rectangle from the edges.
    /// This works even when size() is used unlike stock MQ styles.
    #[allow(dead_code)]
    pub fn padding(self, padding: RectOffset) -> Self {
        GroupStyle { padding, ..self }
    }

    /// Set the background image to use
    #[allow(dead_code)]
    pub fn background(self, background: Image) -> Self {
        GroupStyle { background: Some(background), ..self }
    }

    /// Set the border color to use
    #[allow(dead_code)]
    pub fn border_color(self, color: Color) -> Self {
        GroupStyle { border_color: Some(color), ..self }
    }

    /// Create the macroquad skin from the group style
    fn skin(&self) -> Skin {
        // This is a work-around for Macroquad's lack of relative positioning for windows.
        // By using a button with a background image and a group for layout we can mimic
        // the base window functionality while providing relative positioning.
        // NO_COLOR gets rid of the default group 1px white border for group
        // and solid white fill for the button
        let border_color = self.border_color.unwrap_or(NO_COLOR);
        let group_style =
            root_ui().style_builder().color(border_color).color_hovered(NO_COLOR).color_clicked(NO_COLOR).build();
        let button_style = if let Some(bkg) = &self.background {
            root_ui().style_builder().background(bkg.clone()).build()
        } else {
            root_ui().style_builder().color(NO_COLOR).color_hovered(NO_COLOR).color_clicked(NO_COLOR).build()
        };
        Skin { group_style, button_style, ..root_ui().default_skin() }
    }
}

pub struct Group {
    id: Id,
    size: Vec2,
    skin: Skin,
    style: GroupStyle,
    position: Position,
}

impl Group {
    /// Create a new group instance
    pub fn new(id: Id, size: Vec2, style: GroupStyle) -> Self {
        Group { id, size, skin: style.skin(), style, position: Position::default() }
    }

    /// Configure initial position of the group on the screen
    #[allow(dead_code)]
    pub fn position<T: Into<Position>>(self, pos: T) -> Self {
        Group { position: pos.into(), ..self }
    }

    /// Return the size of the group
    #[allow(dead_code)]
    pub fn size(&self) -> Vec2 {
        self.size
    }

    /// Draw the group. The callback `f` will be called with the current `Ui` instance and
    /// the available content size of the group as arguments
    pub fn ui<F: FnOnce(&mut Ui, Vec2)>(&self, ui: &mut Ui, f: F) {
        ui.push_skin(&self.skin);

        // Calculate desired position
        let bg_position = match self.position {
            Position::Center => vec2(screen_width() - self.size.x, screen_height() - self.size.y) / 2.0,
            Position::CenterTop => vec2(screen_width() - self.size.x, 0.0) / 2.0,
            Position::Absolute(position) => position,
        };

        // Draw button as workaround for background image
        widgets::Button::new("").size(self.size).position(bg_position).ui(ui);

        // Calculate group size and position taking padding into account.
        // Padding reduces the group size and shifts position to even it out.
        let group_size = vec2(
            self.size.x - self.style.padding.left - self.style.padding.right,
            self.size.y - self.style.padding.top - self.style.padding.bottom,
        );
        let group_position = vec2(bg_position.x + self.style.padding.left, bg_position.y + self.style.padding.top);

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
