//! TitleBar encapsulates and automates the manipulation of a set of widgets to provide
//! typical title bar type functionality.
use crate::{
    group::{Group, GroupStyle},
    menu::{Menu, MenuStyle},
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

    pub options_btn_bg: Image,     // button image to use for options
    pub options_btn_clk_bg: Image, // button image to use for menu when clicked
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

    // Return the MQ Style for the options button
    fn options_btn(&self) -> Style {
        root_ui()
            .style_builder()
            .background(self.options_btn_bg.clone())
            .background_hovered(self.options_btn_bg.clone())
            .background_clicked(self.options_btn_clk_bg.clone())
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
    id: Id,                   // title bar identifier
    skin: Skin,               // caching the macroquad object
    style: TitleBarStyle,     // access to raw styling resources
    group: Group,             // access to underlying group
    title: String,            // title for the title bar
    title_position: Position, // position for the title
    options: bool,            // true if the options button was pressed
}

impl TitleBar {
    /// Create a new instance
    pub fn new<T: AsRef<str>>(id: Id, title: T, style: TitleBarStyle) -> Self {
        let group_style = GroupStyle::new().border_color(BLUE);
        let group = Group::new(id, style.size(), group_style).position(Position::CenterTop);
        let skin =
            Skin { button_style: style.options_btn(), label_style: style.title(), ..root_ui().default_skin() };
        TitleBar {
            id,
            skin,
            style,
            group,
            title: title.as_ref().to_string(),
            title_position: Position::default(),
            options: false,
        }
    }

    /// Returns true if the options menu should be displayed
    pub fn options(&self) -> bool {
        return self.options;
    }

    /// Position the title on the title bar
    pub fn title_position<T: Into<Position>>(self, pos: T) -> Self {
        TitleBar { title_position: pos.into(), ..self }
    }

    /// Draw the title bar and associated ui elements on the screen
    pub fn ui(&mut self, ui: &mut Ui) {
        // Draw the title bar
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

            // Draw options button and save state
            let options_size = vec2(title_size.y, title_size.y);
            let options_pos =
                vec2(size.x - options_size.x - self.style.padding.right, (size.y - options_size.y) / 2.0);
            if widgets::Button::new("").size(options_size).position(options_pos).ui(ui) {
                self.options = !self.options
            }
            ui.pop_skin();
        });
    }
}
