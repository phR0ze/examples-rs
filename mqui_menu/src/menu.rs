//! Menu encapsulates and automates the manipulation of a set of widgets to provide
//! typical menu type functionality.
use crate::{
    button::Button,
    group::Group,
    position::Position,
    size::{Size, Width},
    utils::*,
};
use macroquad::{
    color::colors,
    hash,
    prelude::*,
    ui::{root_ui, widgets, Id, Skin, Ui},
};

#[derive(Debug, Clone)]
pub struct Menu {
    group: Group, // underly group for positioning, size and layout
    update: bool, // track if a skin update is needed

    // Entries
    entries: Vec<Button>,          // entries for menu
    entry_spacing: f32,            // spacing between menu entries
    entry_clicked: Option<String>, // track if the button has been clicked

    // Shared button property overrides
    entry_width: Option<Width>,        // width of the entry
    entry_padding: RectOffset,         // button inside is padded before allowing content
    entry_position: Position,          // position of entries relative to button
    entry_bg: Option<Image>,           // optional background image to use for button buttons
    entry_bg_clk: Option<Image>,       // background image to use for clicked button buttons
    entry_bg_color: Option<Color>,     // background color to use for entries when background image is not set
    entry_font: Option<&'static [u8]>, // font to use for button text
    entry_font_color: Color,           // font color to use for button text
    entry_font_size: u16,              // font size to use for button text
    entry_label_position: Position,    // position of the label within the button
    entry_icon: Option<Texture2D>,     // optional icon to display
    entry_icon_position: Position,     // positionf of the icon within the button
}

impl Default for Menu {
    fn default() -> Self {
        Menu {
            group: Group::new(),
            update: false,
            entries: vec![],
            entry_spacing: scale(10.),
            entry_clicked: None,
            entry_width: None,
            entry_padding: scale_rect(0.0, 0.0, 10.0, 10.0),
            entry_position: Position::LeftTop(None),
            entry_bg: None,
            entry_bg_clk: None,
            entry_bg_color: None,
            entry_font: None,
            entry_font_size: scale(DEFAULT_FONT_SIZE) as u16,
            entry_font_color: colors::BLACK,
            entry_label_position: Position::Center(Some(scale_rect(0., 0., 0., 0.))),
            entry_icon: None,
            entry_icon_position: Position::LeftCenter(Some(scale_rect(0., 0., 0., 0.))),
        }
    }
}

impl Menu {
    // Create a new instance
    pub fn new() -> Menu {
        Menu { update: true, entry_font: Some(include_bytes!("../assets/HTOWERT.TTF")), ..Menu::default() }
    }

    // /// Instantiate a new menu to be used for options
    // pub fn menu() -> Menu {
    //     Menu::new()
    //         .size(Size::ThreeQuarter(0.0, -1.0))
    //         .position(Position::Left(None))
    //         .entry_width(Width::ThreeQuarter(None))
    // }

    // /// Instantiate a new menu to be used for options
    // pub fn options() -> Menu {
    //     Menu::new()
    //         .size(Size::HalfWidth(5., 250.))
    //         .position(Position::Right(Some(RectOffset::new(0.0, 5.0, 5.0, 0.0))))
    // }

    /// Add a new entry to the menu
    pub fn add_entry<T: AsRef<str>>(self, title: T) -> Self {
        let mut entries = self.entries.to_vec();
        let entry = Button::new(title);
        entries.push(entry);
        Menu { update: true, entries, ..self }
    }

    /// Set the menu's size
    /// * handles scaling for mobile
    pub fn with_size(self, size: Size) -> Self {
        Menu { group: self.group.with_size(size), ..self }
    }

    /// Position the menu on the screen
    pub fn with_position<T: Into<Position>>(self, pos: T) -> Self {
        Menu { group: self.group.with_position(pos), ..self }
    }

    /// Set the background image used for the menu
    pub fn with_background(self, image: Image) -> Self {
        Menu { group: self.group.with_background(image), ..self }
    }

    /// Set the background color used for the menu
    pub fn with_background_color(self, color: Color) -> Self {
        Menu { group: self.group.with_background_color(color), ..self }
    }

    /// Pad inside group pushing content in from edges
    /// * handles scaling for mobile
    pub fn with_padding(self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Menu { group: self.group.with_padding(left, right, top, bottom), ..self }
    }

    /// Set entry background images to use for entries
    pub fn with_entry_images<T: Into<Option<Image>>>(self, regular: T, clicked: T) -> Self {
        Menu { update: true, entry_bg: regular.into(), entry_bg_clk: clicked.into(), ..self }
    }

    /// Set entry background color to use for the entries
    pub fn with_entry_bg_color<T: Into<Option<Color>>>(self, color: T) -> Self {
        Menu { update: true, entry_bg_color: color.into(), ..self }
    }

    /// Set font to use for the entries
    pub fn with_entry_font(self, font: &'static [u8]) -> Self {
        Menu { update: true, entry_font: Some(font), ..self }
    }

    /// Set font size to use for the entries
    /// * handles scaling for mobile
    pub fn with_entry_font_size(self, size: u16) -> Self {
        Menu { update: true, entry_font_size: scale(size as f32) as u16, ..self }
    }

    /// Set font color to use for the entries
    pub fn with_entry_font_color(self, color: Color) -> Self {
        Menu { update: true, entry_font_color: color, ..self }
    }

    /// Set position directive for entries
    /// * handles scaling for mobile
    pub fn with_entry_position(self, pos: Position) -> Self {
        Menu { entry_position: pos.scale(), ..self }
    }

    /// Set padding inside entry around content
    pub fn with_entry_padding(self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Menu { entry_padding: scale_rect(left, right, top, bottom), ..self }
    }

    /// Set the entry width
    pub fn with_entry_width<T: Into<Option<Width>>>(self, width: T) -> Self {
        Menu { entry_width: width.into(), ..self }
    }

    /// Set space between menu entries
    pub fn with_entry_spacing(self, spacing: f32) -> Self {
        Menu { entry_spacing: scale(spacing), ..self }
    }

    /// Returns the entry that was clicked
    pub fn entry_clicked(&self) -> Option<String> {
        match &self.entry_clicked {
            Some(x) => Some(x.clone()),
            None => None,
        }
    }

    /// Update all entries with the latest shared properties
    fn update(&mut self, ui: &mut Ui) {
        if !self.update {
            return;
        }
        for entry in self.entries.iter_mut() {
            entry.width = self.entry_width;
            entry.padding = self.entry_padding;
            entry.position = self.entry_position;
            entry.background = self.entry_bg.as_ref().map(|x| x.clone());
            entry.background_clicked = self.entry_bg_clk.as_ref().map(|x| x.clone());
            entry.background_color = self.entry_bg_color;
            entry.font = self.entry_font;
            entry.font_color = self.entry_font_color;
            entry.font_size = self.entry_font_size;
            entry.label_position = self.entry_label_position;
            entry.icon = self.entry_icon.as_ref().map(|x| x.clone());
            entry.icon_position = self.entry_icon_position;
        }
        self.update = true;
    }

    /// Draw the menu on the screen
    pub fn ui(&mut self, ui: &mut Ui) {
        self.update(ui);
        self.group.ui(ui, |ui, size| {
            // Draw the regular menu entries
            for (i, entry) in self.entries.iter_mut().enumerate() {
                let spacing = if i != 0 { i as f32 * self.entry_spacing } else { 0. };
                //let mut entry_pos = self.entry_position.relative(entry_size, size, None);
                // entry_pos.y += entry_size.y * i as f32 + spacing;
                // if widgets::Button::new("").size(entry_size).position(entry_pos).ui(ui) {
                //     self.entry_clicked = Some(entry.clone());
                // }
                entry.ui(ui, size);

                // Record the button that was clicked
                if entry.clicked() {
                    self.entry_clicked = Some(entry.label.clone());
                }
            }
        });
    }
}
