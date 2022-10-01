//! Button encapsulates and automates the manipulation of a set of widgets to provide
//! typical button type functionality. Macroquad's button implementation doesn't allow
//! for positioning the label. This implementation does.
use core::prelude::*;

#[derive(Debug, Clone)]
pub struct Button {
    skin: Option<Skin>,              // skin to use for the entry titles
    clicked: bool,                   // track button clicked state
    activated: bool,                 // track button activation i.e. odd clicks
    label_size: Vec2,                // calculated size of the label
    dirty: bool,                     // track if a skin update is needed
    width: Option<Width>,            // width of the entry
    offset: Option<RectOffset>,      // offset from the calculated position
    padding: RectOffset,             // button inside is padded before allowing content
    position: Position,              // position of entries relative to button
    background: Option<Image>,       // optional background image to use for button buttons
    background_clk: Option<Image>,   // background image to use for clicked button buttons
    background_color: Option<Color>, // background color to use for entries when background image is not set
    font: Option<&'static [u8]>,     // font to use for button text
    font_color: Color,               // font color to use for button text
    font_size: u16,                  // font size to use for button text
    label: String,                   // button label text value
    label_position: Position,        // position of the label within the button
    icon: Option<Texture2D>,         // optional icon to display
    icon_position: Position,         // positionf of the icon within the button
}

impl Default for Button {
    fn default() -> Self {
        Button {
            skin: None,
            activated: false,
            clicked: false,
            dirty: false,
            label_size: vec2(0., 0.),
            width: None,
            offset: None,
            padding: scale_rect(20., 20., 10., 10.),
            position: Position::default(),
            background: None,
            background_clk: None,
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
        Button { dirty: true, label: label.as_ref().to_string(), ..Button::default() }
            .with_font(Some(include_bytes!("../assets/HTOWERT.TTF")))
    }

    /// Create a new button instance with an icon
    pub fn icon<T: AsRef<str>>(label: T, icon: Texture2D) -> Button {
        Button::new(label)
            .with_position(Position::LeftCenter(None))
            .with_width(Width::ThreeQuarter(0., 0.))
            .with_icon(icon)
            .with_icon_position(Position::LeftCenter(rect(20., 0., 0., 0.)))
            .with_label_position(Position::LeftCenter(rect(80., 0., 3., 0.)))
    }

    /// Set the button's width directive
    pub fn with_width<T: Into<Option<Width>>>(self, width: T) -> Self {
        Button { width: width.into(), ..self }
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

    /// Pad inside widget pushing content in from edges
    /// * handles scaling for mobile
    pub fn with_padding_p(self, padding: RectOffset) -> Self {
        Button { padding: scale_rect_p(padding), ..self }
    }

    /// Set background images to use
    pub fn with_background_images<T: Into<Option<Image>>>(self, regular: T, clicked: T) -> Self {
        Button { dirty: true, background: regular.into(), background_clk: clicked.into(), ..self }
    }

    /// Set the background color used for the button
    pub fn with_background_color<T: Into<Option<Color>>>(self, color: T) -> Self {
        Button { dirty: true, background_color: color.into(), ..self }
    }

    /// Set font to use
    pub fn with_font(self, font: Option<&'static [u8]>) -> Self {
        Button { dirty: true, font, ..self }
    }

    /// Set font size to use for the button label
    /// * handles scaling for mobile
    pub fn with_font_size(self, size: f32) -> Self {
        Button { dirty: true, font_size: scale(size) as u16, ..self }
    }

    /// Set font color to use
    pub fn with_font_color(self, color: Color) -> Self {
        Button { dirty: true, font_color: color, ..self }
    }

    /// Position the label inside the button
    /// * handles scaling for mobile
    pub fn with_label_position(self, pos: Position) -> Self {
        Button { label_position: pos.scale(), ..self }
    }

    /// Set icon to use
    pub fn with_icon<T: Into<Option<Texture2D>>>(self, icon: T) -> Self {
        Button { icon: icon.into(), ..self }
    }

    /// Position the icon inside the button
    /// * handles scaling for mobile
    pub fn with_icon_position(self, pos: Position) -> Self {
        Button { icon_position: pos.scale(), ..self }
    }

    /// Offset the calculated position by this amount
    /// * handles scaling for mobile
    pub fn offset<T: Into<Option<RectOffset>>>(&mut self, offset: T) {
        self.offset = offset.into().map(|x| scale_rect_p(x));
    }

    /// Button label
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Calculate the size based on the width directive inside the given container
    /// * `container` is the containing widget's size to relatively position against
    pub fn size(&self, container: Vec2) -> Vec2 {
        match self.width {
            Some(width) => {
                vec2(width.relative(container), self.label_size.y + self.padding.top + self.padding.bottom)
            },
            None => vec2(
                self.label_size.x + self.padding.left + self.padding.right,
                self.label_size.y + self.padding.top + self.padding.bottom,
            ),
        }
    }

    /// Returns the position of the button
    /// * `container` is the containing widget's size to relatively position against
    pub fn position(&self, container: Vec2) -> Vec2 {
        self.position.relative(self.size(container), container, None)
    }

    /// Returns true if button was clicked an odd number of times. 1st click will activate the
    /// button and the 2nd click will deactivate the button and so on.
    pub fn activated(&self) -> bool {
        self.activated
    }

    /// Returns true if the button was clicked
    pub fn clicked(&self) -> bool {
        self.clicked
    }

    /// Update the skin based on the persisted button properties
    fn update(&mut self, ui: &mut Ui) {
        if !self.dirty {
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
        if let Some(background) = &self.background_clk {
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
        self.dirty = false;
    }

    /// Draw the widget on the screen
    /// * `container` is the containing widget's size to relatively position against
    pub fn ui(&mut self, ui: &mut Ui, container: Vec2) {
        self.update(ui);
        ui.push_skin(self.skin.as_ref().unwrap());

        // Draw button
        let btn_size = self.size(container);
        let mut btn_pos = self.position(container);
        if let Some(offset) = self.offset {
            btn_pos.x += offset.left - offset.right;
            btn_pos.y += offset.top - offset.bottom;
        }
        if widgets::Button::new("").size(btn_size).position(btn_pos).ui(ui) {
            self.activated = !self.activated;
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
