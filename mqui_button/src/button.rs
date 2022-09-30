//! Button encapsulates and automates the manipulation of a set of widgets to provide
//! typical button type functionality.
use crate::{position::Position, size::Width, utils::*};
use macroquad::{
    color::colors,
    prelude::*,
    ui::{root_ui, widgets, Skin, Ui},
};

#[derive(Debug, Clone)]
pub struct Button {
    skin: Option<Skin>,                // skin to use for the entry titles
    width: Option<Width>,              // width of the entry
    toggle: bool,                      // toggle the button's activation
    clicked: bool,                     // track if the button has been clicked
    padding: RectOffset,               // button inside is padded before allowing content
    position: Position,                // position of entries relative to button
    background: Option<Image>,         // optional background image to use for button buttons
    background_clicked: Option<Image>, // background image to use for clicked button buttons
    background_color: Option<Color>,   // background color to use for entries when background image is not set
    font: Option<Vec<u8>>,             // font to use for button text
    font_color: Color,                 // font color to use for button text
    font_size: u16,                    // font size to use for button text
    label: String,                     // button label text value
    label_size: Vec2,                  // calculated size of the label
    label_position: Position,          // position of the label within the button
    icon: Option<Texture2D>,           // optional icon to display
    icon_position: Position,           // positionf of the icon within the button
}

impl Default for Button {
    fn default() -> Self {
        Button {
            skin: None,
            width: None,
            toggle: false,
            clicked: false,
            padding: scale_rect(20., 20., 10., 10.),
            position: Position::default(),
            background: None,
            background_clicked: None,
            background_color: None,
            font: None,
            font_size: scale(DEFAULT_FONT_SIZE) as u16,
            font_color: colors::BLACK,
            label: String::default(),
            label_size: vec2(0., 0.),
            label_position: Position::Center(Some(scale_rect(0., 0., 0., 0.))),
            icon: None,
            icon_position: Position::LeftCenter(Some(scale_rect(0., 0., 0., 0.))),
        }
    }
}

impl Button {
    /// Create a new standard button instance
    pub fn new<T: AsRef<str>>(label: T) -> Button {
        Button { label: label.as_ref().to_string(), ..Button::default() }
            .font(include_bytes!("../assets/HTOWERT.TTF"))
            .update_skin()
    }

    /// Create a new button instance with an icon
    pub fn icon<T: AsRef<str>>(label: T, icon: Image) -> Button {
        Button::new(label)
            .position(Position::LeftCenter(None))
            .width(Width::ThreeQuarter(0., 0.))
            .icon_image(icon)
            .icon_position(Position::LeftCenter(rect(20., 0., 0., 0.)))
            .label_position(Position::LeftCenter(rect(80., 0., 3., 0.)))
    }

    /// Set the button's width directive
    pub fn width(self, width: Width) -> Self {
        Button { width: Some(width), ..self }
    }

    /// Position the button on the screen
    /// * handles scaling for mobile
    pub fn position(self, pos: Position) -> Self {
        Button { position: pos.scale(), ..self }
    }

    /// Pad inside widget pushing content in from edges
    /// * handles scaling for mobile
    pub fn padding(self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Button { padding: scale_rect(left, right, top, bottom), ..self }
    }

    /// Set background images to use
    pub fn background_images(self, regular: Image, clicked: Image) -> Self {
        Button { background: Some(regular), background_clicked: Some(clicked), ..self }.update_skin()
    }

    /// Set the background color used for the button
    pub fn background_color(self, color: Color) -> Self {
        Button { background_color: Some(color), ..self }.update_skin()
    }

    /// Set font to use
    pub fn font(self, font: &[u8]) -> Self {
        Button { font: Some(font.to_vec()), ..self }.update_skin()
    }

    /// Set font size to use for the button label
    /// * handles scaling for mobile
    pub fn font_size(self, size: f32) -> Self {
        Button { font_size: scale(size) as u16, ..self }.update_skin()
    }

    /// Set font color to use
    pub fn font_color(self, color: Color) -> Self {
        Button { font_color: color, ..self }.update_skin()
    }

    /// Position the label inside the button
    /// * handles scaling for mobile
    pub fn label_position(self, pos: Position) -> Self {
        Button { label_position: pos.scale(), ..self }
    }

    /// Set icon image to use
    pub fn icon_image(self, icon: Image) -> Self {
        Button { icon: Some(Texture2D::from_image(&icon)), ..self }
    }

    /// Position the icon inside the button
    /// * handles scaling for mobile
    pub fn icon_position(self, pos: Position) -> Self {
        Button { icon_position: pos.scale(), ..self }
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
    fn update_skin(self) -> Self {
        // Create the label style
        let mut style = root_ui()
            .style_builder()
            .text_color(self.font_color)
            .text_color_hovered(self.font_color)
            .text_color_clicked(self.font_color)
            .font_size(self.font_size);
        if let Some(font) = &self.font {
            style = style.font(font).unwrap();
        }
        let label_style = style.build();

        // Create the button style
        style = root_ui().style_builder();
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
        let skin = Skin { button_style, label_style, ..root_ui().default_skin() };

        // Calculate text size and include margin
        let label_size = text_size(&skin, Some(&self.label));

        Button { label_size, skin: Some(skin), ..self }
    }

    /// Draw the widget on the screen
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.push_skin(self.skin.as_ref().unwrap());

        // Calculate size and position
        let btn_size = match self.width {
            Some(width) => vec2(width.f32(), self.label_size.y + self.padding.top + self.padding.bottom),
            None => vec2(
                self.label_size.x + self.padding.left + self.padding.right,
                self.label_size.y + self.padding.top + self.padding.bottom,
            ),
        };

        // Draw button for catching clicks without label as
        // macroquad button's don't allow for positioning the label
        let btn_pos = self.position.vec2(btn_size);
        if widgets::Button::new("").size(btn_size).position(btn_pos).ui(ui) {
            self.toggle = !self.toggle;
            self.clicked = true;
        }

        // Draw the button icon if set
        if let Some(icon) = &self.icon {
            let icon_size = vec2(self.label_size.y, self.label_size.y);
            let icon_pos = self.icon_position.relative(icon_size, btn_size, Some(btn_pos));
            widgets::Texture::new(*icon).size(icon_size.x, icon_size.y).position(icon_pos).ui(ui);
        }

        // Draw our own label over the top of the button for positioning
        let label_pos = self.label_position.relative(self.label_size, btn_size, Some(btn_pos));
        widgets::Label::new(self.label.as_str()).size(self.label_size).position(label_pos).ui(ui);

        ui.pop_skin();
    }
}
