//! Group provides a relatively positioned MQ group that will adjust to maintain its
//! original position relative to the app window size. This is a work around for the
//! stock MQ window that has a static position regardless of application window size
//! changes.
//!
//! Inspiration for this work-around comes from https://github.com/fishfolks/jumpy
use core::prelude::*;

#[derive(Debug, Clone)]
pub struct Group {
    update: bool,                // track if the group needs updated before drawing
    size: Size,                  // size of the group on the screen
    position: Position,          // position the group on the screen
    padding: RectOffset,         // pad inside group pushing content in from edges
    background: Option<Image>,   // optional background image to use, takes priority over background color
    background_color: Color,     // background color to use if background is not set
    border_color: Option<Color>, // optional border color to use
    scrolling: bool,             // enable scrolling when true
    skin: Option<Skin>,          // cached MQ skin for drawing
}

impl Group {
    /// Create a new group instance
    pub fn new() -> Self {
        Group {
            update: true,
            size: Size::default(),
            position: Position::default(),
            padding: RectOffset::default(),
            background: None,
            background_color: colors::GRAY,
            border_color: None,
            scrolling: false,
            skin: None,
        }
    }

    /// Set size of the group
    /// * handles scaling for mobile
    pub fn with_size(self, size: Size) -> Self {
        Group { size, ..self }
    }

    /// Set position on the screen
    pub fn with_position<T: Into<Position>>(self, pos: T) -> Self {
        Group { position: pos.into(), ..self }
    }

    /// Pad inside group pushing content in from edges
    /// * handles scaling for mobile
    pub fn with_padding(self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Group { padding: scale_rect(left, right, top, bottom), ..self }
    }

    /// Set the background image to use. Takes priority over background color
    pub fn with_background(self, background: Image) -> Self {
        Group { update: true, background: Some(background), ..self }
    }

    /// Set the background color to use. Only has affect if background image not set
    pub fn with_background_color(self, color: Color) -> Self {
        Group { update: true, background_color: color, ..self }
    }

    /// Set the border color to use
    pub fn with_border_color(self, color: Color) -> Self {
        Group { update: true, border_color: Some(color), ..self }
    }

    /// Set scrolling state
    pub fn with_scrolling(self, scrolling: bool) -> Self {
        Group { update: true, scrolling, ..self }
    }

    /// Update the macroquad skin based on the group's current properties
    fn update(&mut self, ui: &mut Ui) {
        if !self.update {
            return;
        }
        // This is a work-around for Macroquad's lack of relative positioning for windows.
        // By using a button with a background image and a group for layout we can mimic
        // the base window functionality while providing relative positioning.
        // BLANK color gets rid of the default group 1px white border for group
        // and solid white fill for the button
        let border_color = self.border_color.unwrap_or(BLANK);
        let group_style = ui.style_builder().color(border_color).color_hovered(BLANK).color_clicked(BLANK).build();

        let button_style = if let Some(bkg) = &self.background {
            ui.style_builder().background(bkg.clone()).build()
        } else {
            ui.style_builder()
                .color(self.background_color)
                .color_hovered(self.background_color)
                .color_clicked(self.background_color)
                .build()
        };

        // Hide the group scrollbar when content expands beyond the group size
        if !self.scrolling {
            let scroll_width = 0.0;
            let scroll_multiplier = 0.0;
            let scrollbar_style =
                ui.style_builder().color(BLANK).color_hovered(BLANK).color_clicked(BLANK).build();
            let scrollbar_handle_style =
                ui.style_builder().color(BLANK).color_hovered(BLANK).color_clicked(BLANK).build();

            self.skin = Some(Skin {
                group_style,
                button_style,
                scrollbar_style,
                scrollbar_handle_style,
                scroll_width,
                scroll_multiplier,
                ..ui.default_skin()
            });
        } else {
            self.skin = Some(Skin { group_style, button_style, ..ui.default_skin() });
        }
        self.update = false;
    }

    /// Draw the group and call the callback with group's size and position.
    /// * `f` is a callback with params (Ui, size)
    pub fn ui<F: FnOnce(&mut Ui, Vec2)>(&mut self, ui: &mut Ui, f: F) {
        self.update(ui);
        ui.push_skin(self.skin.as_ref().unwrap());

        // Draw button as workaround for background image
        let size = self.size.vec2();
        let position = self.position.vec2(size);
        widgets::Button::new("").size(size).position(position).ui(ui);

        // Calculate group size and position taking padding into account.
        // Padding reduces the group size and shifts position to even it out.
        let group_size =
            vec2(size.x - self.padding.left - self.padding.right, size.y - self.padding.top - self.padding.bottom);
        let group_position = vec2(position.x + self.padding.left, position.y + self.padding.top);

        // Group provides a box to layout out any widgets inside that overlays
        // the non-interactive button.
        widgets::Group::new(hash!(), group_size).position(group_position).ui(ui, |ui| {
            ui.pop_skin();
            f(ui, group_size)
        });

        // Together they form window like functionality that can resize dynamnically
        // based on the application window size changes. MQ's stock window doesn't
        // provide this ability; instead it is static regardless of parent window size
    }
}
