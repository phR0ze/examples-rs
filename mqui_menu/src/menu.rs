//! Menu encapsulates and automates the manipulation of a set of widgets to provide
//! typical menu type functionality.
use crate::{group::Group, position::Position, size::Size, utils::*};
use macroquad::{
    color::colors,
    hash,
    prelude::*,
    ui::{root_ui, widgets, Id, Skin, Style, Ui},
};

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

#[derive(Debug, Clone)]
pub struct Menu {
    id: Id,       // menu identifier
    skin: Skin,   // cached MQ skin for drawing
    group: Group, // underly group for positioning, size and layout

    // Entries
    entry_bg: Option<Image>,           // optional background image to use for menu buttons
    entry_clk_bg: Option<Image>,       // background image to use for clicked menu buttons
    entry_hov_bg: Option<Image>,       // background image to use for hovered menu buttons
    entry_font: Option<&'static [u8]>, // font to use for button text
    entry_font_color: Color,           // font color to use for button text
    entry_font_size: u16,              // font size to use for button text
    entry_padding: RectOffset,         // button inside is padded before allowing content
    entry_spacing: f32,                // space to leave between menu entries
    entries: Vec<MenuEntry>,           // Entries for menu
}

impl Menu {
    // Create a new instance
    pub fn new() -> Menu {
        // Separating out root_ui uses as a runtime borrow issue will occur if
        // we don't allow each usage to complete out before trying to do another
        // operation that depends on root_ui
        let skin = root_ui().default_skin();
        let group = Group::new();

        Menu {
            id: hash!(),
            skin,
            group,
            entry_bg: None,
            entry_clk_bg: None,
            entry_hov_bg: None,
            entry_font: None,
            entry_font_size: scale(30.) as u16,
            entry_font_color: colors::BLACK,
            entry_spacing: scale(10.),
            entry_padding: scale_rect(0.0, 0.0, 10.0, 10.0),
            entries: vec![],
        }
        .update_cached_skin()
    }

    /// Instantiate a new menu to be used for options
    pub fn menu() -> Menu {
        Menu::new().size(Size::ThreeQuarter(0.0, -1.0)).position(Position::TopLeft(None))
    }

    /// Instantiate a new menu to be used for options
    pub fn options() -> Menu {
        Menu::new()
            .size(Size::HalfWidth(5., 250.))
            .position(Position::TopRight(Some(RectOffset::new(0.0, 5.0, 5.0, 0.0))))
    }

    /// Set the menu id
    pub fn id<T: Into<u64>>(self, id: T) -> Self {
        Menu { id: id.into(), ..self }
    }

    /// Add a new entry to the menu
    pub fn add(self, entry: MenuEntry) -> Self {
        let mut entries = self.entries.to_vec();
        entries.push(entry);
        Menu { entries, ..self }
    }

    /// Set the menu's size
    /// * handles scaling for mobile
    pub fn size(self, size: Size) -> Self {
        Menu { group: self.group.size(size), ..self }
    }

    /// Position the menu on the screen
    pub fn position<T: Into<Position>>(self, pos: T) -> Self {
        Menu { group: self.group.position(pos), ..self }
    }

    /// Set the background image used for the menu
    pub fn background(self, image: Image) -> Self {
        Menu { group: self.group.background(image), ..self }.update_cached_skin()
    }

    /// Pad inside group pushing content in from edges
    /// * handles scaling for mobile
    pub fn padding(self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Menu { group: self.group.padding(left, right, top, bottom), ..self }
    }

    /// Set background to use for entries
    pub fn entry_bg(self, image: Image) -> Self {
        Menu { entry_bg: Some(image), ..self }.update_cached_skin()
    }

    /// Set background to use for entries when clicked
    pub fn entry_clk_bg(self, image: Image) -> Self {
        Menu { entry_clk_bg: Some(image), ..self }.update_cached_skin()
    }

    /// Set background to use for entries when hovering
    pub fn entry_hov_bg(self, image: Image) -> Self {
        Menu { entry_hov_bg: Some(image), ..self }.update_cached_skin()
    }

    /// Set font to use for the entries
    pub fn entry_font(self, font: &'static [u8]) -> Self {
        Menu { entry_font: Some(font), ..self }.update_cached_skin()
    }

    /// Set font size to use for the entries
    pub fn entry_font_size(self, size: u16) -> Self {
        Menu { entry_font_size: size as u16, ..self }.update_cached_skin()
    }

    /// Set font color to use for the entries
    pub fn entry_font_color(self, color: Color) -> Self {
        Menu { entry_font_color: color, ..self }.update_cached_skin()
    }

    /// Set padding inside entry around content
    pub fn entry_padding(self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Menu { entry_padding: scale_rect(left, right, top, bottom), ..self }
    }

    /// Set space between menu entries
    pub fn entry_spacing(self, spacing: f32) -> Self {
        Menu { entry_spacing: scale(spacing), ..self }
    }

    /// Update the cached macroquad skin based on the group's current properties
    fn update_cached_skin(self) -> Self {
        let mut style = root_ui()
            .style_builder()
            .text_color(self.entry_font_color)
            .text_color_hovered(self.entry_font_color)
            .text_color_clicked(self.entry_font_color)
            .font_size(self.entry_font_size);
        if let Some(background) = &self.entry_bg {
            style = style.background(background.clone());
        }
        if let Some(background) = &self.entry_hov_bg {
            style = style.background_hovered(background.clone());
        }
        if let Some(background) = &self.entry_clk_bg {
            style = style.background_clicked(background.clone());
        }
        if let Some(font) = self.entry_font {
            style = style.font(font).unwrap();
        }
        let button_style = style.build();
        Menu { skin: Skin { button_style, ..root_ui().default_skin() }, ..self }
    }

    // Return entry height based on font size and padding
    fn entry_height(&self) -> f32 {
        self.entry_font_size as f32 + self.entry_padding.top + self.entry_padding.bottom
    }

    /// Return entry size based on given content size and entry font size
    fn entry_size(&self, content_size: Vec2) -> Vec2 {
        vec2(content_size.x, self.entry_height())
    }

    /// Return entry position based on the given index location and spacing
    fn entry_pos(&self, index: usize) -> Vec2 {
        let spacing = if index != 0 && self.entry_spacing > 0. { index as f32 * self.entry_spacing } else { 0. };
        vec2(0.0, index as f32 * self.entry_height() + spacing)
    }

    /// Draw the menu on the screen
    pub fn ui(&self, ui: &mut Ui) {
        self.group.ui(ui, |ui, size, pos| {
            ui.push_skin(&self.skin);

            // Draw the regular menu entries
            for (i, entry) in self.entries.iter().enumerate() {
                let size = self.entry_size(size);
                let pos = self.entry_pos(i);
                widgets::Button::new(entry.title.as_str()).size(size).position(pos).ui(ui);
            }

            ui.pop_skin();
        });
    }
}
