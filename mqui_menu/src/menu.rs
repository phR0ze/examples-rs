//! Menu encapsulates and automates the manipulation of a set of widgets to provide
//! typical menu type functionality.
use crate::{
    group::{Group, GroupStyle},
    position::Position,
    utils::*,
};
use macroquad::{
    prelude::*,
    ui::{root_ui, widgets, Id, Skin, Style, Ui},
};

#[derive(Debug, Clone)]
pub struct MenuStyle {
    pub background: Option<Image>, // optional image to use as the background for the menu
    pub padding: RectOffset,       // pad inside menu this much from edges before content is allowed

    // Entry style
    pub entry_bg: Option<Image>,     // optional background image to use for menu buttons
    pub entry_clk_bg: Option<Image>, // background image to use for clicked menu buttons
    pub entry_hov_bg: Option<Image>, // background image to use for hovered menu buttons
    pub entry_font: Option<&'static [u8]>, // font to use for button text
    pub entry_font_color: Color,     // font color to use for button text
    pub entry_font_size: u16,        // font size to use for button text
    pub entry_padding: RectOffset,   // button inside is padded before allowing content
    pub entry_spacing: f32,          // space to leave between menu entries
}

impl MenuStyle {
    pub fn new() -> MenuStyle {
        MenuStyle {
            background: None,
            padding: scale_rect(20., 20., 20., 20.),
            entry_bg: None,
            entry_clk_bg: None,
            entry_hov_bg: None,
            entry_font: None,
            entry_font_size: scale(40.) as u16,
            entry_font_color: Color::from_rgba(180, 180, 100, 255),
            entry_spacing: scale(10.),
            entry_padding: scale_rect(0.0, 0.0, 10.0, 10.0),
        }
    }

    /// Set the background image used for the menu
    pub fn background(self, image: Image) -> Self {
        MenuStyle { background: Some(image), ..self }
    }

    /// Set padding inside the menu. Pushes content in from edges
    pub fn padding(self, image: Image) -> Self {
        MenuStyle { background: Some(image), ..self }
    }

    /// Set background to use for entries
    pub fn entry_bg(self, image: Image) -> Self {
        MenuStyle { entry_bg: Some(image), ..self }
    }

    /// Set background to use for entries when clicked
    pub fn entry_clk_bg(self, image: Image) -> Self {
        MenuStyle { entry_clk_bg: Some(image), ..self }
    }

    /// Set background to use for entries when hovering
    pub fn entry_hov_bg(self, image: Image) -> Self {
        MenuStyle { entry_hov_bg: Some(image), ..self }
    }

    /// Set font to use for the entries
    pub fn entry_font(self, font: &'static [u8]) -> Self {
        MenuStyle { entry_font: Some(font), ..self }
    }

    /// Set font size to use for the entries
    pub fn entry_font_size<T: Into<u16>>(self, size: T) -> Self {
        MenuStyle { entry_font_size: size.into(), ..self }
    }

    /// Set font color to use for the entries
    pub fn entry_font_color(self, color: Color) -> Self {
        MenuStyle { entry_font_color: color, ..self }
    }

    /// Set padding inside entry around content
    pub fn entry_padding(self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        MenuStyle { entry_padding: scale_rect(left, right, top, bottom), ..self }
    }

    /// Set space between menu entries
    pub fn entry_spacing(self, spacing: f32) -> Self {
        MenuStyle { entry_spacing: scale(spacing), ..self }
    }

    /// Returns the MQ Style for entries
    pub fn entry(&self) -> Style {
        let mut style = root_ui().style_builder();
        if let Some(background) = &self.background {
            style = style.background(background.clone())
        }
        // if self.background.is_some()
        //     .background(self.entry_bg.clone())
        //     .background_hovered(self.entry_hov_bg.clone())
        //     .background_clicked(self.entry_clk_bg.clone())
        //     .font(self.entry_font)
        //     .unwrap()
        //     .text_color(self.entry_font_color)
        //     .text_color_hovered(self.entry_font_color)
        //     .font_size(self.entry_font_size)
        //     .build()
        style.build()
    }

    // Return entry height based on font size and padding
    pub fn entry_height(&self) -> f32 {
        self.entry_font_size as f32 + self.entry_padding.top + self.entry_padding.bottom
    }

    /// Return entry size based on given content size and entry font size
    pub fn entry_size(&self, content_size: Vec2) -> Vec2 {
        vec2(content_size.x, self.entry_height())
    }

    /// Return entry position based on the given index location and spacing
    pub fn entry_pos(&self, index: usize) -> Vec2 {
        let spacing = if index != 0 && self.entry_spacing > 0. { index as f32 * self.entry_spacing } else { 0. };
        vec2(0.0, index as f32 * self.entry_height() + spacing)
    }
}

#[derive(Debug, Default, Clone)]
pub struct MenuEntry {
    pub title: String,
}

impl MenuEntry {
    /// Create a new instance
    pub fn new<T: AsRef<str>>(title: T) -> Self {
        MenuEntry { title: title.as_ref().to_string() }
    }
}

pub struct Menu {
    id: Id,
    skin: Skin,
    style: MenuStyle,
    group: Group,
    entries: Vec<MenuEntry>,
}

impl Menu {
    /// Create a new instance
    pub fn new(id: Id, size: Vec2, entries: &[MenuEntry], style: MenuStyle) -> Self {
        let mut group_style = GroupStyle::new().padding(style.padding);
        if let Some(background) = &style.background {
            group_style = group_style.background(background.clone());
        }
        let group = Group::new(id, size, group_style).position(Position::Center);

        // Configure menu and entry styles
        let skin = Skin { button_style: style.entry(), ..root_ui().default_skin() };
        Menu { id, skin, style, group, entries: entries.to_vec() }
    }

    /// Position the menu on the screen
    pub fn position<T: Into<Position>>(self, pos: T) -> Self {
        let group = self.group.position(pos);
        Menu { group, ..self }
    }

    /// Draw the menu on the screen
    pub fn ui(&self, ui: &mut Ui) {
        self.group.ui(ui, |ui, size| {
            ui.push_skin(&self.skin);

            // Draw the regular menu entries
            for (i, entry) in self.entries.iter().enumerate() {
                let size = self.style.entry_size(size);
                let pos = self.style.entry_pos(i);
                widgets::Button::new(entry.title.as_str()).size(size).position(pos).ui(ui);
            }

            ui.pop_skin();
        });
    }
}
