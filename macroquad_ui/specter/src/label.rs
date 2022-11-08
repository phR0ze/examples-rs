//! Label encapsulates and extends Macroquad's label supporting:
//! * Calculated sizing and positioning relative to containing widget
//! * Builder for reusable layout but also direct modification
use crate::prelude::*;

/// Label builder provides a template for building new labels with a persisted reusable
/// configuration
#[derive(Debug, Clone)]
pub struct LabelBuilder {
    layout: Layout,                // layout
    font: Option<&'static [u8]>,   // font to use for label
    font_size: f32,                // font size to use for label
    font_color: Color,             // font color to use for label
    font_color_clk: Option<Color>, // font color to use for label when clicked
    font_color_hov: Option<Color>, // font color to use for label when hovered
}

impl LabelBuilder {
    /// Create a new builder instance
    pub fn new() -> Self {
        Self {
            layout: Layout::new(""),
            font: None,
            font_size: scale(DEFAULT_FONT_SIZE),
            font_color: colors::BLACK,
            font_color_clk: None,
            font_color_hov: None,
        }
    }

    /// Set the layout identifier
    pub fn id<T: AsRef<str>>(self, id: T) -> Self {
        Self {
            layout: self.layout.with_id(id),
            ..self
        }
    }

    /// Set font to use
    pub fn font(self, font: Option<&'static [u8]>) -> Self {
        Self { font, ..self }
    }

    /// Set font size to use for the button label
    /// * handles scaling for mobile
    pub fn font_size(self, size: f32) -> Self {
        Self {
            font_size: size,
            ..self
        }
    }

    /// Set font color to use
    pub fn font_color(self, color: Color) -> Self {
        Self {
            font_color: color,
            ..self
        }
    }

    /// Set font color to use when clicked
    pub fn font_color_clk(self, color: Color) -> Self {
        Self {
            font_color_clk: Some(color),
            ..self
        }
    }

    /// Set font color to use when hovered
    pub fn font_color_hov(self, color: Color) -> Self {
        Self {
            font_color_hov: Some(color),
            ..self
        }
    }

    /// Set layout to use
    pub fn layout<F: FnOnce(Layout) -> Layout>(self, f: F) -> Self {
        Self {
            layout: f(self.layout),
            ..self
        }
    }

    /// Create a new button instance
    pub fn build<T: AsRef<str>>(&self, text: T) -> Label {
        let conf = self.clone().layout(|x| x.copy().with_id(text.as_ref()));
        Label {
            conf,
            dirty: true,
            skin: None,
            text: text.as_ref().to_string(),
        }
    }
}

/// Label encapsulates and extends Macroquad's label
#[derive(Debug, Clone)]
pub struct Label {
    conf: LabelBuilder, // configuration
    dirty: bool,        // track if a skin update is needed
    skin: Option<Skin>, // skin to use
    text: String,       // actual text to display
}

// Constructors and builder functions
impl Label {
    /// Create a new widget instance
    pub fn new<T: AsRef<str>>(text: T) -> Self {
        LabelBuilder::new().build(text)
    }

    /// Set the layout's identifier
    pub fn id<T: AsRef<str>>(self, id: T) -> Self {
        Self {
            dirty: true,
            conf: self.conf.id(id),
            ..self
        }
    }

    /// Set font to use
    pub fn font(self, font: Option<&'static [u8]>) -> Self {
        Self {
            dirty: true,
            conf: self.conf.font(font),
            ..self
        }
    }

    /// Set font size to use for the button label
    /// * handles scaling for mobile
    pub fn font_size(self, size: f32) -> Self {
        Self {
            dirty: true,
            conf: self.conf.font_size(size),
            ..self
        }
    }

    /// Set font color to use
    pub fn font_color(self, color: Color) -> Self {
        Self {
            dirty: true,
            conf: self.conf.font_color(color),
            ..self
        }
    }

    /// Set font color to use when clicked
    pub fn font_color_clk(self, color: Color) -> Self {
        Self {
            dirty: true,
            conf: self.conf.font_color_clk(color),
            ..self
        }
    }

    /// Set font color to use when hovered
    pub fn font_color_hov(self, color: Color) -> Self {
        Self {
            dirty: true,
            conf: self.conf.font_color_hov(color),
            ..self
        }
    }

    /// Set layout properties to use
    pub fn layout<F: FnOnce(Layout) -> Layout>(self, f: F) -> Self {
        Self {
            dirty: true,
            conf: self.conf.layout(f),
            ..self
        }
    }
}

// Utility functions
impl Label {
    /// Get a reference to the layout
    pub fn get_layout(&self) -> &Layout {
        &self.conf.layout
    }

    /// Set the widget's text value
    pub fn set_text<T: AsRef<str>>(&mut self, text: T) {
        self.dirty = true;
        self.text = text.as_ref().to_string();
    }

    /// Get the widget's shape from its layout
    pub fn shape(&self) -> (Vec2, Vec2) {
        self.conf.layout.shape()
    }

    /// Get the widget's text value
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Make layout, styling and shape calculation updates in prepartion for showing
    /// * Note: will be called automatically in most cases. Only useful to call when composing
    /// other widgets from this widget
    pub fn ui(&mut self, ui: &mut Ui) {
        if !self.dirty {
            return;
        }
        let mut style = ui.style_builder().text_color(self.conf.font_color).font_size(self.conf.font_size as u16);
        if let Some(color) = self.conf.font_color_clk {
            style = style.text_color_clicked(color);
        }
        if let Some(color) = self.conf.font_color_hov {
            style = style.text_color_hovered(color);
        }
        if let Some(font) = self.conf.font {
            style = style.font(font).unwrap();
        }
        let label_style = style.build();

        // Create the skin based on the two override styles
        let skin = Skin {
            label_style,
            ..ui.default_skin()
        };

        // Calculate text size and include margin
        self.conf.layout.set_size(text_size(ui, &skin, Some(&self.text)));
        self.skin = Some(skin);
        self.dirty = false;
    }

    /// Draw the widget on the screen
    /// * `layout` parent layout to draw button within
    /// * returns true when clicked in the current frame
    pub fn show(&mut self, ui: &mut Ui, layout: Option<&Layout>) {
        self.ui(ui);
        ui.push_skin(self.skin.as_ref().unwrap());

        // Set parent if given
        if let Some(parent) = layout {
            parent.append(&self.conf.layout);
        }

        let (pos, size) = self.conf.layout.shape();
        widgets::Label::new(self.text.as_str()).size(size).position(pos).ui(ui);

        ui.pop_skin();
    }
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_foo() {
        assert_eq!(vec2(2., 2.), vec2(2., 2.));
    }
}
