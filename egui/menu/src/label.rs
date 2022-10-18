//! Label encapsulates and extends Macroquad's button supporting:
//! * Calculated sizing and positioning relative to containing widget
use crate::prelude::*;

/// LabelBuilder provides the ability to preserve widget configuration and be able to repeatedly
/// create new widget instances based on this configuration rather than have to configure each
/// individual widget instance.
#[derive(Debug, Clone)]
pub struct LabelBuilder {
    size: Option<Size>,          // sizing of the widget
    position: Position,          // position of of the widget
    font: Option<&'static [u8]>, // font to use for button text
    font_color: Color,           // font color to use
    font_color_clk: Color,       // font color to use when clicked
    font_color_hov: Color,       // font color to use when hovered
    font_size: u16,              // font size to use for button text
    margin: RectOffset,          // allow for this space around widget content
}

impl LabelBuilder {
    /// Create a new builder instance
    pub fn new() -> Self {
        Self {
            size: None,
            position: Position::Center(Some(RectOffset::default())),
            font: None,
            font_size: scale(DEFAULT_FONT_SIZE) as u16,
            font_color: colors::BLACK,
            font_color_clk: colors::BLACK,
            font_color_hov: colors::BLACK,
            margin: RectOffset::default(),
        }
    }

    /// Set size of the group
    /// * handles scaling for mobile
    pub fn size<T: Into<Option<Size>>>(self, size: T) -> Self {
        Self { size: size.into(), ..self }
    }

    /// Set position on the screen
    pub fn position(self, position: Position) -> Self {
        Self { position, ..self }
    }

    /// Set font to use
    pub fn font(self, font: Option<&'static [u8]>) -> Self {
        Self { font, ..self }
    }

    /// Set font size to use
    /// * handles scaling for mobile
    pub fn font_size(self, size: f32) -> Self {
        Self { font_size: scale(size) as u16, ..self }
    }

    /// Set font color to use
    pub fn font_color(self, color: Color) -> Self {
        Self { font_color: color, ..self }
    }

    /// Set font color to use when clicked
    pub fn font_color_clk(self, color: Color) -> Self {
        Self { font_color_clk: color, ..self }
    }

    /// Set font color to use when hovered
    pub fn font_color_hov(self, color: Color) -> Self {
        Self { font_color_hov: color, ..self }
    }

    /// Create a new widget instance from this builder
    pub fn build<T: AsRef<str>>(&self, text: T) -> Label {
        Label {
            dirty: true,
            conf: self.clone(),
            skin: None,
            text: text.as_ref().to_string(),
            calc_size: vec2(0., 0.),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Label {
    dirty: bool,        // track if a skin update is needed
    skin: Option<Skin>, // skin to use
    conf: LabelBuilder, // widget configuration
    text: String,       // actual text to display
    calc_size: Vec2,    // calculated text size
}

impl Label {
    /// Create a new widget instance
    pub fn new<T: AsRef<str>>(text: T) -> Self {
        Self {
            dirty: true,
            skin: None,
            conf: LabelBuilder::new(),
            text: text.as_ref().to_string(),
            calc_size: vec2(0., 0.),
        }
    }

    /// Set size of the group
    /// * handles scaling for mobile
    pub fn with_size<T: Into<Option<Size>>>(self, size: T) -> Self {
        Self { conf: LabelBuilder { size: size.into(), ..self.conf }, ..self }
    }

    /// Set position on the screen
    pub fn with_position(self, position: Position) -> Self {
        Self { conf: LabelBuilder { position, ..self.conf }, ..self }
    }

    /// Set font to use
    pub fn with_font(self, font: Option<&'static [u8]>) -> Self {
        Self { dirty: true, conf: LabelBuilder { font, ..self.conf }, ..self }
    }

    /// Set font size to use
    /// * handles scaling for mobile
    pub fn with_font_size(self, size: f32) -> Self {
        Self { dirty: true, conf: LabelBuilder { font_size: scale(size) as u16, ..self.conf }, ..self }
    }

    /// Set font color to use
    pub fn with_font_color(self, color: Color) -> Self {
        Self { dirty: true, conf: LabelBuilder { font_color: color, ..self.conf }, ..self }
    }

    /// Set font color to use when clicked
    pub fn with_font_color_clk(self, color: Color) -> Self {
        Self { dirty: true, conf: LabelBuilder { font_color_clk: color, ..self.conf }, ..self }
    }

    /// Set font color to use when hovered
    pub fn with_font_color_hov(self, color: Color) -> Self {
        Self { dirty: true, conf: LabelBuilder { font_color_hov: color, ..self.conf }, ..self }
    }

    /// Get the widget's text value
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Update the skin based on the persisted button properties
    fn update_skin(&mut self, ui: &mut Ui) {
        if !self.dirty {
            return;
        }
        let mut style = ui
            .style_builder()
            .text_color(self.conf.font_color)
            .text_color_hovered(self.conf.font_color)
            .text_color_clicked(self.conf.font_color);
        if let Some(font) = self.conf.font {
            style = style.font(font).unwrap();
        }
        let label_style = style.build();

        // Create the skin based on the two override styles
        let skin = Skin { label_style, ..ui.default_skin() };

        // Calculate text size and include margin
        self.calc_size = text_size(ui, &skin, Some(&self.text));
        self.skin = Some(skin);
        self.dirty = false;
    }

    /// Draw the widget on the screen
    /// * `cont_size` is the containing widget's size
    /// * `offset` any positional offset to take into account
    pub fn ui(&mut self, ui: &mut Ui, cont_size: Vec2, offset: Option<Vec2>) {
        self.update_skin(ui);
        ui.push_skin(self.skin.as_ref().unwrap());

        let pos = self.conf.position.relative(self.calc_size, cont_size, offset);
        widgets::Label::new(self.text.as_str()).size(self.calc_size).position(pos).ui(ui);

        ui.pop_skin();
    }
}
