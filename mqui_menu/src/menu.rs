//! Menu encapsulates and automates the manipulation of a set of widgets to provide
//! typical menu type functionality.
use crate::{group::Group, position::Position, size::Size, utils::*};
use macroquad::{
    color::colors,
    hash,
    prelude::*,
    ui::{root_ui, widgets, Id, Skin, Ui},
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
    id: Id,             // menu identifier
    group: Group,       // underly group for positioning, size and layout
    skin: Option<Skin>, // cached MQ skin for drawing

    // Entries
    entry_bg: Option<Image>,           // optional background image to use for menu buttons
    entry_clk_bg: Option<Image>,       // background image to use for clicked menu buttons
    entry_bg_color: Option<Color>,     // background color to use for entries when background image is not set
    entry_font: Option<&'static [u8]>, // font to use for button text
    entry_font_color: Color,           // font color to use for button text
    entry_font_size: u16,              // font size to use for button text
    entry_padding: RectOffset,         // button inside is padded before allowing content
    entry_spacing: f32,                // space to leave between menu entries
    entry_position: Position,          // position of entries relative to menu
    entry_size: Option<Size>,          // size of the entry
    entries: Vec<MenuEntry>,           // entries for menu
    entry_clicked: Option<MenuEntry>,  // track if an entry has been clicked
}

impl Default for Menu {
    fn default() -> Self {
        Menu {
            id: hash!(),
            skin: None,
            group: Group::new(),
            entry_bg: None,
            entry_clk_bg: None,
            entry_bg_color: None,
            entry_font: None,
            entry_font_size: scale(DEFAULT_FONT_SIZE) as u16,
            entry_font_color: colors::BLACK,
            entry_spacing: scale(10.),
            entry_padding: scale_rect(0.0, 0.0, 10.0, 10.0),
            entry_position: Position::Left(None),
            entry_size: None,
            entries: vec![],
            entry_clicked: None,
        }
    }
}

impl Menu {
    // Create a new instance
    pub fn new() -> Menu {
        Menu::default().entry_font(include_bytes!("../assets/HTOWERT.TTF"))
    }

    /// Instantiate a new menu to be used for options
    pub fn menu() -> Menu {
        let menu = Menu::new().size(Size::ThreeQuarter(0.0, -1.0)).position(Position::Left(None));
        let entry_height = text_height(menu.skin.as_ref()) + menu.entry_padding.top + menu.entry_padding.bottom;
        menu.entry_size(Size::ThreeQuarter(0.0, entry_height))
    }

    /// Instantiate a new menu to be used for options
    pub fn options() -> Menu {
        Menu::new()
            .size(Size::HalfWidth(5., 250.))
            .position(Position::Right(Some(RectOffset::new(0.0, 5.0, 5.0, 0.0))))
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

    /// Set the background color used for the menu
    pub fn background_color(self, color: Color) -> Self {
        Menu { group: self.group.background_color(color), ..self }.update_cached_skin()
    }

    /// Pad inside group pushing content in from edges
    /// * handles scaling for mobile
    pub fn padding(self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Menu { group: self.group.padding(left, right, top, bottom), ..self }
    }

    /// Set entry background images to use for entries
    pub fn entry_images(self, regular: Image, clicked: Image) -> Self {
        Menu { entry_bg: Some(regular), entry_clk_bg: Some(clicked), ..self }.update_cached_skin()
    }

    /// Set entry background color to use for the entries
    pub fn entry_bg_color(self, color: Color) -> Self {
        Menu { entry_bg_color: Some(color), ..self }.update_cached_skin()
    }

    /// Set font to use for the entries
    pub fn entry_font(self, font: &'static [u8]) -> Self {
        Menu { entry_font: Some(font), ..self }.update_cached_skin()
    }

    /// Set font size to use for the entries
    /// * handles scaling for mobile
    pub fn entry_font_size(self, size: u16) -> Self {
        Menu { entry_font_size: scale(size as f32) as u16, ..self }.update_cached_skin()
    }

    /// Set font color to use for the entries
    pub fn entry_font_color(self, color: Color) -> Self {
        Menu { entry_font_color: color, ..self }.update_cached_skin()
    }

    /// Set padding inside entry around content
    pub fn entry_padding(self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Menu { entry_padding: scale_rect(left, right, top, bottom), ..self }
    }

    /// Set the entry size. The desired font must be set at this point to get an accurate
    /// * handles scaling for mobile
    pub fn entry_size(self, size: Size) -> Self {
        Menu { entry_size: Some(size), ..self }
    }

    /// Set space between menu entries
    pub fn entry_spacing(self, spacing: f32) -> Self {
        Menu { entry_spacing: scale(spacing), ..self }
    }

    /// Returns the entry that was clicked
    pub fn entry_clicked(&self) -> Option<MenuEntry> {
        match &self.entry_clicked {
            Some(x) => Some(x.clone()),
            None => None,
        }
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
        if let Some(background) = &self.entry_clk_bg {
            style = style.background_clicked(background.clone());
        }
        if let Some(color) = &self.entry_bg_color {
            style = style.color(*color).color_clicked(*color).color_hovered(*color);
        }
        if let Some(font) = self.entry_font {
            style = style.font(font).unwrap();
        }
        let button_style = style.build();
        Menu { skin: Some(Skin { button_style, ..root_ui().default_skin() }), ..self }
    }

    /// Draw the menu on the screen
    pub fn ui(&mut self, ui: &mut Ui) {
        self.group.ui(ui, |ui, size| {
            ui.push_skin(self.skin.as_ref().unwrap());

            // Draw the regular menu entries
            for (i, entry) in self.entries.iter().enumerate() {
                let entry_size = match self.entry_size {
                    Some(x) => x.vec2(),
                    None => vec2(100., 100.),
                };

                // Calculate entry position
                let spacing = if i != 0 && self.entry_spacing > 0. { i as f32 * self.entry_spacing } else { 0. };
                let mut entry_pos = self.entry_position.relative(entry_size, size);
                entry_pos.y += entry_size.y * i as f32 + spacing;
                // if widgets::Button::new("").size(entry_size).position(entry_pos).ui(ui) {
                //     self.entry_clicked = Some(entry.clone());
                // }
                widgets::Label::new(entry.title.as_str()).position(entry_pos);
            }

            ui.pop_skin();
        });
    }
}
