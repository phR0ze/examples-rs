//! TitleBar encapsulates and automates the manipulation of a set of widgets to provide
//! typical title bar type functionality.
use crate::{
    group::{Group, GroupStyle},
    position::Position,
};
use macroquad::{
    prelude::*,
    ui::{root_ui, widgets, Id, Skin, Style, Ui},
};

#[derive(Debug, Clone)]
pub struct TitleBarStyle {
    pub padding: RectOffset, // padding to apply around title

    pub title_font: &'static [u8], // font to use for the title
    pub title_font_size: u16,      // font size for the title
    pub title_font_color: Color,   // font color for the title

    pub settings_btn_bg: Image,     // button image to use for settings
    pub settings_btn_clk_bg: Image, // button image to use for menu when clicked
}

impl TitleBarStyle {
    // Return the MQ Style for the title
    fn title(&self) -> Style {
        root_ui()
            .style_builder()
            .font(self.title_font)
            .unwrap()
            .text_color(self.title_font_color)
            .text_color_hovered(self.title_font_color)
            .text_color_clicked(self.title_font_color)
            .font_size(self.title_font_size)
            .build()
    }

    // Return the MQ Style for the settings button
    fn settings(&self) -> Style {
        root_ui()
            .style_builder()
            .background(self.settings_btn_bg.clone())
            .background_hovered(self.settings_btn_bg.clone())
            .background_clicked(self.settings_btn_clk_bg.clone())
            .build()
    }

    /// Return title bar height based on title font size and padding
    pub fn height(&self) -> f32 {
        self.title_font_size as f32 + self.padding.top + self.padding.bottom
    }

    /// Return title bar size based on screen width, font size and padding
    pub fn size(&self) -> Vec2 {
        vec2(screen_width(), self.height())
    }
}

pub struct TitleBar {
    id: Id,        // title bar identifier
    skin: Skin,    // caching the macroquad object
    title: String, // title for the title bar
    title_position: Position,
    style: TitleBarStyle, // access to raw styling resources
    group: Group,         // access to underlying group
}

impl TitleBar {
    /// Create a new instance
    pub fn new<T: AsRef<str>>(id: Id, title: T, style: TitleBarStyle) -> Self {
        let group_style = GroupStyle::new().border_color(BLUE);
        let group = Group::new(id, style.size(), group_style).position(Position::CenterTop);
        let skin = Skin { label_style: style.title(), button_style: style.settings(), ..root_ui().default_skin() };
        TitleBar { id, skin, style, title: title.as_ref().to_string(), title_position: Position::default(), group }
    }

    /// Position the title on the title bar
    pub fn title_position<T: Into<Position>>(self, pos: T) -> Self {
        TitleBar { title_position: pos.into(), ..self }
    }

    /// Draw the menu on the screen
    pub fn ui(&self, ui: &mut Ui) {
        self.group.ui(ui, |ui, size| {
            ui.push_skin(&self.skin);

            // Draw title
            let title_size = ui.calc_size(&self.title);
            let title_position = match self.title_position {
                Position::Center => vec2(size.x - title_size.x, size.y - title_size.y) / 2.0,
                Position::CenterTop => vec2(size.x - title_size.x, 0.0) / 2.0,
                Position::Absolute(position) => position,
            };
            ui.label(title_position, &self.title);

            // Draw settings
            let settings_size = vec2(title_size.y, title_size.y);
            let settings_pos =
                vec2(size.x - settings_size.x - self.style.padding.right, (size.y - settings_size.y) / 2.0);
            widgets::Button::new("").size(settings_size).position(settings_pos).ui(ui);

            ui.pop_skin();
        });
    }
}
