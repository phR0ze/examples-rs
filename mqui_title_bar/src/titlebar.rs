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

    pub menu_btn: Image,        // button image to use
    pub menu_btn_clk: Image,    // button image to use when clicked
    pub options_btn: Image,     // button image to use
    pub options_btn_clk: Image, // button image to use when clicked
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

    // Return the MQ Style for the menu button
    fn menu_btn(&self) -> Style {
        root_ui()
            .style_builder()
            .background(self.menu_btn.clone())
            .background_hovered(self.menu_btn.clone())
            .background_clicked(self.menu_btn_clk.clone())
            .build()
    }

    // Return the MQ Style for the options button
    fn options_btn(&self) -> Style {
        root_ui()
            .style_builder()
            .background(self.options_btn.clone())
            .background_hovered(self.options_btn.clone())
            .background_clicked(self.options_btn_clk.clone())
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
    style: TitleBarStyle,     // access to raw styling resources
    group: Group,             // access to underlying group
    title: String,            // title for the title bar
    title_position: Position, // position for the title
    title_skin: Skin,         // title skin
    menu: bool,               // true if the menu button was pressed
    menu_skin: Skin,          // menu button skin
    options: bool,            // true if the options button was pressed
    options_skin: Skin,       // options button skin
}

impl TitleBar {
    /// Create a new instance
    pub fn new<T: AsRef<str>>(id: Id, title: T, style: TitleBarStyle) -> Self {
        let group_style = GroupStyle::new().border_color(BLUE);
        let group = Group::new(id, style.size(), group_style).position(Position::CenterTop);
        let title_skin = Skin { label_style: style.title(), ..root_ui().default_skin() };
        let menu_skin = Skin { button_style: style.menu_btn(), ..root_ui().default_skin() };
        let options_skin = Skin { button_style: style.options_btn(), ..root_ui().default_skin() };
        TitleBar {
            id,
            style,
            group,
            title_skin,
            title: title.as_ref().to_string(),
            title_position: Position::default(),
            menu: false,
            menu_skin,
            options: false,
            options_skin,
        }
    }

    /// Returns true if the menu should be displayed
    pub fn menu(&self) -> bool {
        return self.menu;
    }

    /// Returns true if the options should be displayed
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
            // Draw title
            ui.push_skin(&self.title_skin);
            let title_size = ui.calc_size(&self.title);
            let title_position = match self.title_position {
                Position::Center => vec2(size.x - title_size.x, size.y - title_size.y) / 2.0,
                Position::CenterTop => vec2(size.x - title_size.x, 0.0) / 2.0,
                Position::Absolute(position) => position,
            };
            ui.label(title_position, &self.title);
            ui.pop_skin();

            // Draw menu button and save state
            ui.push_skin(&self.menu_skin);
            let menu_size = vec2(title_size.y, title_size.y);
            let menu_pos = vec2(self.style.padding.left, (size.y - menu_size.y) / 2.0);
            if widgets::Button::new("").size(menu_size).position(menu_pos).ui(ui) {
                self.menu = !self.menu
            }
            ui.pop_skin();

            // Draw options button and save state
            ui.push_skin(&self.options_skin);
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
