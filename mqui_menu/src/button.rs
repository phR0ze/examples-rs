//! Button encapsulates and automates the manipulation of a set of widgets to provide
//! typical button type functionality.
use crate::{
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
pub struct Button {
    entry_bg: Option<Image>,            // optional background image to use for button buttons
    entry_clk_bg: Option<Image>,        // background image to use for clicked button buttons
    entry_bg_color: Option<Color>,      // background color to use for entries when background image is not set
    entry_font: Option<&'static [u8]>,  // font to use for button text
    entry_font_color: Color,            // font color to use for button text
    entry_font_size: u16,               // font size to use for button text
    entry_padding: RectOffset,          // button inside is padded before allowing content
    entry_spacing: f32,                 // space to leave between button entries
    entry_position: Position,           // position of entries relative to button
    entry_width: Option<Width>,         // width of the entry
    entries: Vec<ButtonEntry>,          // entries for button
    entry_clicked: Option<ButtonEntry>, // track if an entry has been clicked
    entry_title_skin: Option<Skin>,     // skin to use for the entry titles
    entry_button_skin: Option<Skin>,    // cached MQ skin for drawing
    entry_title_height: f32,            // height of entry text to account for in sizing
}

impl Default for Button {
    fn default() -> Self {
        Button {
            id: hash!(),
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
            entry_width: None,
            entries: vec![],
            entry_clicked: None,
            entry_title_skin: None,
            entry_button_skin: None,
            entry_title_height: scale(DEFAULT_FONT_SIZE),
        }
    }
}

impl Button {
    // Create a new instance
    pub fn new() -> Button {
        Button::default().entry_font(include_bytes!("../assets/HTOWERT.TTF")).update_entry_button_skin()
    }

    /// Instantiate a new button to be used for options
    pub fn button() -> Button {
        Button::new()
            .size(Size::ThreeQuarter(0.0, -1.0))
            .position(Position::Left(None))
            .entry_width(Width::ThreeQuarter(None))
    }

    /// Instantiate a new button to be used for options
    pub fn options() -> Button {
        Button::new()
            .size(Size::HalfWidth(5., 250.))
            .position(Position::Right(Some(RectOffset::new(0.0, 5.0, 5.0, 0.0))))
    }

    /// Set the button id
    pub fn id<T: Into<u64>>(self, id: T) -> Self {
        Button { id: id.into(), ..self }
    }

    /// Add a new entry to the button
    pub fn add(self, entry: ButtonEntry) -> Self {
        let mut entries = self.entries.to_vec();
        entries.push(entry);
        Button { entries, ..self }
    }

    /// Set the button's size
    /// * handles scaling for mobile
    pub fn size(self, size: Size) -> Self {
        Button { group: self.group.size(size), ..self }
    }

    /// Position the button on the screen
    pub fn position<T: Into<Position>>(self, pos: T) -> Self {
        Button { group: self.group.position(pos), ..self }
    }

    /// Set the background image used for the button
    pub fn background(self, image: Image) -> Self {
        Button { group: self.group.background(image), ..self }.update_entry_button_skin()
    }

    /// Set the background color used for the button
    pub fn background_color(self, color: Color) -> Self {
        Button { group: self.group.background_color(color), ..self }.update_entry_button_skin()
    }

    /// Pad inside group pushing content in from edges
    /// * handles scaling for mobile
    pub fn padding(self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Button { group: self.group.padding(left, right, top, bottom), ..self }
    }

    /// Set entry background images to use for entries
    pub fn entry_images(self, regular: Image, clicked: Image) -> Self {
        Button { entry_bg: Some(regular), entry_clk_bg: Some(clicked), ..self }.update_entry_button_skin()
    }

    /// Set entry background color to use for the entries
    pub fn entry_bg_color(self, color: Color) -> Self {
        Button { entry_bg_color: Some(color), ..self }.update_entry_button_skin()
    }

    /// Set font to use for the entries
    pub fn entry_font(self, font: &'static [u8]) -> Self {
        Button { entry_font: Some(font), ..self }.update_entry_title_skin()
    }

    /// Set font size to use for the entries
    /// * handles scaling for mobile
    pub fn entry_font_size(self, size: u16) -> Self {
        Button { entry_font_size: scale(size as f32) as u16, ..self }.update_entry_button_skin()
    }

    /// Set font color to use for the entries
    pub fn entry_font_color(self, color: Color) -> Self {
        Button { entry_font_color: color, ..self }.update_entry_button_skin()
    }

    /// Set padding inside entry around content
    pub fn entry_padding(self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Button { entry_padding: scale_rect(left, right, top, bottom), ..self }
    }

    /// Set the entry width
    pub fn entry_width(self, width: Width) -> Self {
        Button { entry_width: Some(width), ..self }
    }

    /// Set space between button entries
    pub fn entry_spacing(self, spacing: f32) -> Self {
        Button { entry_spacing: scale(spacing), ..self }
    }

    /// Returns the entry that was clicked
    pub fn entry_clicked(&self) -> Option<ButtonEntry> {
        match &self.entry_clicked {
            Some(x) => Some(x.clone()),
            None => None,
        }
    }

    /// Update the entry title skin based on the persisted button properties
    fn update_entry_title_skin(self) -> Self {
        let mut style = root_ui()
            .style_builder()
            .text_color(self.entry_font_color)
            .text_color_hovered(self.entry_font_color)
            .text_color_clicked(self.entry_font_color)
            .font_size(self.entry_font_size);
        if let Some(font) = self.entry_font {
            style = style.font(font).unwrap();
        }
        let label_style = style.build();
        let skin = Skin { label_style, ..root_ui().default_skin() };
        let entry_title_height = text_height(&skin);
        Button { entry_title_height, entry_title_skin: Some(skin), ..self }
    }

    /// Update the entry button skin based on the persisted button properties
    fn update_entry_button_skin(self) -> Self {
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
        Button { entry_button_skin: Some(Skin { button_style, ..root_ui().default_skin() }), ..self }
    }

    /// Draw the button on the screen
    pub fn ui(&mut self, ui: &mut Ui) {
        self.group.ui(ui, |ui, size| {
            // Draw the regular button entries
            for (i, entry) in self.entries.iter().enumerate() {
                let entry_size = match self.entry_width {
                    Some(x) => {
                        vec2(x.f32(), self.entry_title_height + self.entry_padding.top + self.entry_padding.bottom)
                    },
                    None => vec2(100., 100.),
                };

                // Calculate entry position
                let spacing = if i != 0 && self.entry_spacing > 0. { i as f32 * self.entry_spacing } else { 0. };
                let mut entry_pos = self.entry_position.relative(entry_size, size);
                entry_pos.y += entry_size.y * i as f32 + spacing;
                ui.push_skin(self.entry_button_skin.as_ref().unwrap());
                if widgets::Button::new("").size(entry_size).position(entry_pos).ui(ui) {
                    self.entry_clicked = Some(entry.clone());
                }
                ui.pop_skin();
                ui.push_skin(self.entry_title_skin.as_ref().unwrap());
                widgets::Label::new(entry.title.as_str()).position(entry_pos).ui(ui);
                ui.pop_skin();
            }
        });
    }
}
