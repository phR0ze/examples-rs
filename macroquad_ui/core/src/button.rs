//! Button encapsulates and extends Macroquad's button supporting:
//! * Border color for regular, clicked and hovered
//! * Icon support with positioning and sizing inside button
//!
//! * Button activated toggle
//! * Label positioning inside the button
//! * Calculated sizing and positioning relative to containing widget
use crate::prelude::*;

const ICON_ID: &'static str = "icon";
const LABEL_ID: &'static str = "label";

/// Button encapsulates and extends Macroquad's button
#[derive(Debug, Clone)]
pub struct Button {
    dirty: bool,        // track if a skin update is needed
    skin: Option<Skin>, // skin to use for the entry titles
    label: String,      // button label text value
    clicked: bool,      // track button clicked state
    activated: bool,    // track button activation i.e. odd clicks

    // Configuration
    layout: Layout,
    background: Option<Image>,           // background image to use
    background_clk: Option<Image>,       // background image to use when clicked
    background_hov: Option<Image>,       // background image to use when hovered
    background_color: Color,             // background color
    background_color_clk: Option<Color>, // background color when clicked
    background_color_hov: Option<Color>, // background color when hovered
    label_font: Option<&'static [u8]>,   // font to use for label
    label_font_size: f32,                // font size to use for label
    label_font_color: Color,             // font color to use for label
    label_font_color_clk: Option<Color>, // font color to use for label when clicked
    label_font_color_hov: Option<Color>, // font color to use for label when hovered
    icon: Option<Texture2D>,             // optional icon to display
}

// Constructors and builder functions
impl Button {
    /// Create a new standard button instance
    pub fn new<T: AsRef<str>>(label: T) -> Self {
        let button = Self {
            dirty: true,
            skin: None,
            label: label.as_ref().to_string(),
            clicked: false,
            activated: false,
            layout: Layout::new(""),
            background: None,
            background_clk: None,
            background_hov: None,
            background_color: colors::BLANK,
            background_color_clk: None,
            background_color_hov: None,
            label_font: None,
            label_font_size: scale(DEFAULT_FONT_SIZE),
            label_font_color: colors::BLACK,
            label_font_color_clk: None,
            label_font_color_hov: None,
            icon: None,
        };

        // Add the label layout by default
        button.layout.append_sub(LABEL_ID, None);
        button
    }

    /// Create a new button instance with an icon
    /// * the icon will be scaled to match the font size
    /// * `label` is the text to display as the button label
    /// * `icon` is a texture to be displayed as the button icon
    pub fn icon<T: AsRef<str>>(label: T, icon: Texture2D) -> Self {
        Button::new(label)
            .with_icon(icon)
            .with_icon_layout(|x| x.with_align(Align::Center).with_margins(10., 0., 0., 0.))
            .with_label_layout(|x| x.with_align(Align::Center).with_margins(10., 10., 2., 0.))
    }

    /// Set background image to use
    pub fn with_background<T: Into<Option<Image>>>(self, image: T) -> Self {
        Button { dirty: true, background: image.into(), ..self }
    }

    /// Set background image to use
    pub fn with_background_clk<T: Into<Option<Image>>>(self, image: T) -> Self {
        Button { dirty: true, background_clk: image.into(), ..self }
    }

    /// Set background image to use
    pub fn with_background_hov<T: Into<Option<Image>>>(self, image: T) -> Self {
        Button { dirty: true, background_hov: image.into(), ..self }
    }

    /// Set the background color used for the button
    pub fn with_background_color(self, color: Color) -> Self {
        Button { dirty: true, background_color: color, ..self }
    }

    /// Set icon to use
    pub fn with_icon<T: Into<Option<Texture2D>>>(self, icon: T) -> Self {
        self.layout.prepend_sub(ICON_ID, None);
        Button { dirty: true, icon: icon.into(), ..self }
    }

    /// Update icon layout properties
    pub fn with_icon_layout<F: FnOnce(Layout) -> Layout>(self, f: F) -> Self {
        self.layout.set_sub(f(self.layout.sub(ICON_ID).unwrap().clone()));
        Button { dirty: true, ..self }
    }

    /// Set font to use
    pub fn with_label_font(self, font: Option<&'static [u8]>) -> Self {
        Button { dirty: true, label_font: font, ..self }
    }

    /// Set font size to use for the button label
    /// * handles scaling for mobile
    pub fn with_label_font_size(self, size: f32) -> Self {
        Button { dirty: true, label_font_size: size, ..self }
    }

    /// Set font color to use
    pub fn with_label_font_color(self, color: Color) -> Self {
        Button { dirty: true, label_font_color: color, ..self }
    }

    /// Set label layout to use
    pub fn with_label_layout<F: FnOnce(Layout) -> Layout>(self, f: F) -> Self {
        self.layout.set_sub(f(self.layout.sub(LABEL_ID).unwrap().clone()));
        Button { dirty: true, ..self }
    }

    /// Set layout to use
    pub fn with_layout<F: FnOnce(Layout) -> Layout>(self, f: F) -> Self {
        Button { dirty: true, layout: f(self.layout), ..self }
    }
}

// Helper functions
impl Button {
    /// Button label
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns true if button was clicked an odd number of times. 1st click will activate the
    /// button and the 2nd click will deactivate the button and so on.
    /// * Button must be instantiated outside main loop for this to work correctly
    pub fn activated(&self) -> bool {
        self.activated
    }

    /// Returns true if the button was clicked
    pub fn clicked(&self) -> bool {
        self.clicked
    }

    /// Prepare to draw the widgets such as skin updates and sizing calculations
    fn ui(&mut self, ui: &mut Ui) {
        if !self.dirty {
            return;
        }
        // Create the label style
        let mut style =
            ui.style_builder().text_color(self.label_font_color).font_size(self.label_font_size as u16);
        if let Some(color) = self.label_font_color_clk {
            style = style.text_color_clicked(color);
        }
        if let Some(color) = self.label_font_color_hov {
            style = style.text_color_hovered(color);
        }
        if let Some(font) = self.label_font {
            style = style.font(font).unwrap();
        }
        let label_style = style.build();

        // Create the button style
        style = ui
            .style_builder()
            .color(self.background_color)
            .color_clicked(self.background_color)
            .color_hovered(self.background_color);
        if let Some(background) = &self.background {
            style = style.background(background.clone());
        }
        if let Some(background) = &self.background_clk {
            style = style.background_clicked(background.clone());
        }
        if let Some(background) = &self.background_hov {
            style = style.background_hovered(background.clone());
        }
        if let Some(color) = &self.background_color_clk {
            style = style.color_clicked(*color);
        }
        if let Some(color) = &self.background_color_hov {
            style = style.color_hovered(*color);
        }
        let button_style = style.build();

        // Create the skin based on override styles
        let skin = Skin { button_style, label_style, ..ui.default_skin() };

        // Calculate and cache button component sizes to reduce compute time
        let label_size = text_size(ui, &skin, Some(&self.label));
        if let Some(_) = &self.icon {
            self.layout.set_sub_size_s(ICON_ID, label_size.y + 5.0, label_size.y + 5.0);
        }
        self.layout.set_sub_size_p(LABEL_ID, label_size);
        self.layout.update();

        self.skin = Some(skin);
        self.dirty = false;
    }

    /// Draw the widget on the screen
    /// * `layout` parent layout to draw button within
    /// * returns true when clicked in the current frame
    pub fn show(&mut self, ui: &mut Ui, layout: Option<&Layout>) -> bool {
        self.ui(ui);
        ui.push_skin(self.skin.as_ref().unwrap());
        self.clicked = false; // reset clicked

        // Set parent if given
        if let Some(layout) = layout {
            self.layout.set_parent(layout)
        }

        // Draw button
        let (pos, size) = self.layout.shape();
        if widgets::Button::new("").size(size).position(pos).ui(ui) {
            self.activated = !self.activated;
            self.clicked = true;
        }

        // Draw icon
        if let Some(icon) = &self.icon {
            let (icon_pos, icon_size) = self.layout.sub_shape(ICON_ID).unwrap();
            widgets::Texture::new(*icon).size(icon_size.x, icon_size.y).position(icon_pos).ui(ui);
        }

        // Draw label
        let (label_pos, label_size) = self.layout.sub_shape(LABEL_ID).unwrap();
        widgets::Label::new(self.label.as_str()).size(label_size).position(label_pos).ui(ui);

        ui.pop_skin();

        self.clicked
    }
}
