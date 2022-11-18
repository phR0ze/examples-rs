//! Label encapsulates and extends Macroquad's label supporting:
//! * Calculated sizing and positioning relative to containing widget
//! * Builder for reusable layout but also direct modification
use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Label {
    skin: Option<Skin>,          // skin to use
    text: String,                // actual text to display
    layout: Layout,              // layout
    font: Option<&'static [u8]>, // font to use for label
    size: f32,                   // font size to use for label
    color: Color,                // font color to use for label
    color_clk: Option<Color>,    // font color to use for label when clicked
    color_hov: Option<Color>,    // font color to use for label when hovered
}

impl Default for Label {
    fn default() -> Self {
        Self {
            skin: None,
            text: "".to_string(),
            layout: Layout::new(""),
            font: None,
            size: scale(DEFAULT_FONT_SIZE),
            color: colors::BLACK,
            color_clk: None,
            color_hov: None,
        }
    }
}

// Constructors and builders
impl Label {
    /// Create a new widget instance
    pub fn new<T: AsRef<str>>(id: T, text: T) -> Self {
        Self::default().id(id).text(text)
    }

    /// Create a new unique instance
    /// * similar to clone but allows for setting unique information
    /// * clones underlying components so there are no ties back to the orignal
    pub fn build<T: AsRef<str>>(&self, id: T, text: T) -> Self {
        self.clone().layout(|x| x.id(id)).text(text)
    }

    /// Set the widget's id
    pub fn id<T: AsRef<str>>(self, id: T) -> Self {
        self.layout(|x| x.id(id))
    }

    /// Set font to use
    pub fn font(self, font: Option<&'static [u8]>) -> Self {
        Self { font, ..self }
    }

    /// Set font size to use for the button label
    /// * handles scaling for mobile
    pub fn size(self, size: f32) -> Self {
        Self { size, ..self }
    }

    /// Set font color to use
    pub fn color(self, color: Color) -> Self {
        Self { color, ..self }
    }

    /// Set font color to use when clicked
    pub fn color_clk(self, color: Color) -> Self {
        Self {
            color_clk: Some(color),
            ..self
        }
    }

    /// Set font color to use when hovered
    pub fn color_hov(self, color: Color) -> Self {
        Self {
            color_hov: Some(color),
            ..self
        }
    }

    /// Set the widget's layout properties
    pub fn layout<F: FnOnce(Layout) -> Layout>(self, f: F) -> Self {
        Self {
            layout: f(self.layout),
            ..self
        }
    }

    /// Set the widget's text value
    pub fn text<T: AsRef<str>>(self, text: T) -> Self {
        Self {
            text: text.as_ref().to_string(),
            ..self
        }
    }
}

// Getters
impl Label {
    /// Get the widget's text value
    pub fn get_text(&self) -> &str {
        &self.text
    }
}

// Setters
impl Label {
    /// Set the widget's text value
    pub fn set_text<T: AsRef<str>>(&mut self, text: T) {
        self.text = text.as_ref().to_string();
    }
}

// Utility functions
impl Label {
    /// Make layout, styling and shape calculation updates in prepartion for showing
    /// * Note: will be called automatically in most cases. Only useful to call when composing
    /// other widgets from this widget
    pub fn pre_calc(&mut self, ui: &mut Ui) -> Vec2 {
        // Create skin
        let mut style = ui.style_builder().text_color(self.color).font_size(self.size as u16);
        if let Some(font) = self.font {
            style = style.font(font).unwrap();
        }
        let label_style = style.build();
        let skin = Skin {
            label_style,
            ..ui.default_skin()
        };

        // Calculate text size
        let size = text_size(ui, &skin, Some(&self.text));
        self.layout.set_size(size.x, size.y);
        self.skin = Some(skin);

        size
    }
}

impl Widget for Label {
    /// Returns a reference clone to the Widget's layout
    fn layout_ptr(&self) -> Layout {
        self.layout.ptr()
    }

    /// Draw the widget on the screen
    /// * `ui` is the Macroquad Ui engine
    fn show_p(&mut self, ui: &mut Ui) -> Response {
        self.pre_calc(ui);
        ui.push_skin(self.skin.as_ref().unwrap());
        let (pos, size) = self.layout.shape();
        widgets::Label::new(self.text.as_str()).size(size).position(pos).ui(ui);
        ui.pop_skin();

        Response::default()
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
