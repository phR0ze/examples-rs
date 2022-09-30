//! Button encapsulates and automates the manipulation of a set of widgets to provide
//! typical button type functionality. Macroquad's button implementation doesn't allow
//! for positioning the label. This implementation does.
use crate::{position::Position, size::Width, utils::*};
use macroquad::{
    color::colors,
    prelude::*,
    ui::{root_ui, widgets, Skin, Ui},
};

#[derive(Debug, Clone)]
pub struct Button {
    skin: Option<Skin>,                           // skin to use for the entry titles
    toggle: bool,                                 // toggle the button's activation
    clicked: bool,                                // track if the button has been clicked
    label_size: Vec2,                             // calculated size of the label
    update: bool,                                 // track if a skin update is needed
    pub(crate) width: Option<Width>,              // width of the entry
    pub(crate) padding: RectOffset,               // button inside is padded before allowing content
    pub(crate) position: Position,                // position of entries relative to button
    pub(crate) background: Option<Image>,         // optional background image to use for button buttons
    pub(crate) background_clicked: Option<Image>, // background image to use for clicked button buttons
    pub(crate) background_color: Option<Color>, // background color to use for entries when background image is not set
    pub(crate) font: Option<&'static [u8]>,     // font to use for button text
    pub(crate) font_color: Color,               // font color to use for button text
    pub(crate) font_size: u16,                  // font size to use for button text
    pub(crate) label: String,                   // button label text value
    pub(crate) label_position: Position,        // position of the label within the button
    pub(crate) icon: Option<Texture2D>,         // optional icon to display
    pub(crate) icon_position: Position,         // positionf of the icon within the button
}

impl Default for Button {
    fn default() -> Self {
        Button {
            skin: None,
            toggle: false,
            clicked: false,
            update: false,
            label_size: vec2(0., 0.),
            width: None,
            padding: scale_rect(20., 20., 10., 10.),
            position: Position::default(),
            background: None,
            background_clicked: None,
            background_color: None,
            font: None,
            font_size: scale(DEFAULT_FONT_SIZE) as u16,
            font_color: colors::BLACK,
            label: String::default(),
            label_position: Position::Center(Some(scale_rect(0., 0., 0., 0.))),
            icon: None,
            icon_position: Position::LeftCenter(Some(scale_rect(0., 0., 0., 0.))),
        }
    }
}

impl Button {
    /// Create a new standard button instance
    pub fn new<T: AsRef<str>>(label: T) -> Button {
        Button { update: true, label: label.as_ref().to_string(), ..Button::default() }
            .with_font(include_bytes!("../assets/HTOWERT.TTF"))
    }

    /// Create a new button instance with an icon
    pub fn icon<T: AsRef<str>>(label: T, icon: Image) -> Button {
        Button::new(label)
            .with_position(Position::LeftCenter(None))
            .with_width(Width::ThreeQuarter(0., 0.))
            .with_icon_image(icon)
            .with_icon_position(Position::LeftCenter(rect(20., 0., 0., 0.)))
            .with_label_position(Position::LeftCenter(rect(80., 0., 3., 0.)))
    }

    /// Set the button's width directive
    pub fn with_width(self, width: Width) -> Self {
        Button { width: Some(width), ..self }
    }

    /// Position the button on the screen
    /// * handles scaling for mobile
    pub fn with_position(self, pos: Position) -> Self {
        Button { position: pos.scale(), ..self }
    }

    /// Pad inside widget pushing content in from edges
    /// * handles scaling for mobile
    pub fn with_padding(self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Button { padding: scale_rect(left, right, top, bottom), ..self }
    }

    /// Set background images to use
    pub fn with_background_images<T: Into<Option<Image>>>(self, regular: T, clicked: T) -> Self {
        Button { update: true, background: regular.into(), background_clicked: clicked.into(), ..self }
    }

    /// Set the background color used for the button
    pub fn with_background_color<T: Into<Option<Color>>>(self, color: T) -> Self {
        Button { update: true, background_color: color.into(), ..self }
    }

    /// Set font to use
    pub fn with_font(self, font: &'static [u8]) -> Self {
        Button { update: true, font: Some(font), ..self }
    }

    /// Set font size to use for the button label
    /// * handles scaling for mobile
    pub fn with_font_size(self, size: f32) -> Self {
        Button { update: true, font_size: scale(size) as u16, ..self }
    }

    /// Set font color to use
    pub fn with_font_color(self, color: Color) -> Self {
        Button { update: true, font_color: color, ..self }
    }

    /// Position the label inside the button
    /// * handles scaling for mobile
    pub fn with_label_position(self, pos: Position) -> Self {
        Button { label_position: pos.scale(), ..self }
    }

    /// Set icon image to use
    pub fn with_icon_image<T: Into<Option<Image>>>(self, image: T) -> Self {
        let icon = image.into().map(|x| Texture2D::from_image(&x));
        Button { icon, ..self }
    }

    /// Position the icon inside the button
    /// * handles scaling for mobile
    pub fn with_icon_position(self, pos: Position) -> Self {
        Button { icon_position: pos.scale(), ..self }
    }

    /// Position the button on the screen
    /// * handles scaling for mobile
    pub fn position(&mut self, pos: Position) {
        self.position = pos.scale();
    }

    /// Returns true if toggle is on the on mode
    pub fn toggle(&self) -> bool {
        self.toggle
    }

    /// Returns true if the button was clicked
    pub fn clicked(&self) -> bool {
        self.clicked
    }

    /// Update the skin based on the persisted button properties
    fn update(&mut self, ui: &mut Ui) {
        if !self.update {
            return;
        }
        // Create the label style
        let mut style = ui
            .style_builder()
            .text_color(self.font_color)
            .text_color_hovered(self.font_color)
            .text_color_clicked(self.font_color)
            .font_size(self.font_size);
        if let Some(font) = self.font {
            style = style.font(font).unwrap();
        }
        let label_style = style.build();

        // Create the button style
        style = ui.style_builder();
        if let Some(background) = &self.background {
            style = style.background(background.clone());
        }
        if let Some(background) = &self.background_clicked {
            style = style.background_clicked(background.clone());
        }
        if let Some(color) = &self.background_color {
            style = style.color(*color).color_clicked(*color).color_hovered(*color);
        }
        let button_style = style.build();

        // Create the skin based on the two override styles
        let skin = Skin { button_style, label_style, ..ui.default_skin() };

        // Calculate text size and include margin
        self.label_size = text_size(ui, &skin, Some(&self.label));
        self.skin = Some(skin);
        self.update = false;
    }

    /// Draw the widget on the screen
    /// * `size` is the containing widget's size to relatively position against
    pub fn ui(&mut self, ui: &mut Ui, size: Vec2) {
        self.update(ui);
        ui.push_skin(self.skin.as_ref().unwrap());

        // Draw button
        let btn_size = match self.width {
            Some(width) => vec2(width.f32(), self.label_size.y + self.padding.top + self.padding.bottom),
            None => vec2(
                self.label_size.x + self.padding.left + self.padding.right,
                self.label_size.y + self.padding.top + self.padding.bottom,
            ),
        };
        let btn_pos = self.position.relative(btn_size, size, None);
        if widgets::Button::new("").size(btn_size).position(btn_pos).ui(ui) {
            self.toggle = !self.toggle;
            self.clicked = true;
        }

        // Draw icon
        if let Some(icon) = &self.icon {
            let icon_size = vec2(self.label_size.y, self.label_size.y);
            let icon_pos = self.icon_position.relative(icon_size, btn_size, Some(btn_pos));
            widgets::Texture::new(*icon).size(icon_size.x, icon_size.y).position(icon_pos).ui(ui);
        }

        // Draw label
        let label_pos = self.label_position.relative(self.label_size, btn_size, Some(btn_pos));
        widgets::Label::new(self.label.as_str()).size(self.label_size).position(label_pos).ui(ui);

        ui.pop_skin();
    }
}
