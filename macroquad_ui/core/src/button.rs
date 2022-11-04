//! Button encapsulates and extends Macroquad's button supporting:
//! * Border color for regular, clicked and hovered states
//! * Label positioning and sizing inside the button
//! * Icon support with positioning and sizing inside button
//! * Button activated toggle
//! * Calculated sizing and positioning relative to containing widget
//! * Builder for reusable layout but also direct modification
use crate::prelude::*;

const ICON_ID: &'static str = "icon";
const LABEL_ID: &'static str = "label";

/// Button builder provides a template for building new buttons with a persisted reusable
/// configuration
#[derive(Debug, Clone)]
pub struct ButtonBuilder {
    layout: Layout,                      // layout
    image: Option<Image>,                // background image to use
    image_clk: Option<Image>,            // background image to use when clicked
    image_hov: Option<Image>,            // background image to use when hovered
    fill: Color,                         // background color
    fill_clk: Option<Color>,             // background color when clicked
    fill_hov: Option<Color>,             // background color when hovered
    label: Label,                        // label widget
    label_font: Option<&'static [u8]>,   // font to use for label
    label_font_size: f32,                // font size to use for label
    label_font_color: Color,             // font color to use for label
    label_font_color_clk: Option<Color>, // font color to use for label when clicked
    label_font_color_hov: Option<Color>, // font color to use for label when hovered
    icon: Option<Texture2D>,             // optional icon to display
}

impl ButtonBuilder {
    /// Create a new builder instance
    pub fn new() -> Self {
        let button = Self {
            layout: Layout::horz(""),
            image: None,
            image_clk: None,
            image_hov: None,
            fill: colors::BLANK,
            fill_clk: None,
            fill_hov: None,
            label: Label::new("").layout(|x| x.id(LABEL_ID)),
            label_font: None,
            label_font_size: scale(DEFAULT_FONT_SIZE),
            label_font_color: colors::BLACK,
            label_font_color_clk: None,
            label_font_color_hov: None,
            icon: None,
        };

        // Add the label layout by default
        button.layout.alloc_append(LABEL_ID, None);
        button
    }

    /// Create a new button instance with an icon
    /// * the icon will be scaled to match the font size
    /// * `icon` is a texture to be displayed as the button icon
    pub fn icon(icon: Texture2D) -> Self {
        ButtonBuilder::new()
            .icon_texture(icon)
            .icon_layout(|x| x.align(Align::Center).margins(10., 10., 5., 5.))
            .label_layout(|x| x.align(Align::Center).margins(10., 10., 0., 0.))
    }

    /// Set background image to use
    pub fn image<T: Into<Option<Image>>>(self, image: T) -> Self {
        Self { image: image.into(), ..self }
    }

    /// Set background image to use
    pub fn image_clk<T: Into<Option<Image>>>(self, image: T) -> Self {
        Self { image_clk: image.into(), ..self }
    }

    /// Set background image to use
    pub fn image_hov<T: Into<Option<Image>>>(self, image: T) -> Self {
        Self { image_hov: image.into(), ..self }
    }

    /// Set the background color used for the button
    pub fn fill(self, color: Color) -> Self {
        Self { fill: color, ..self }
    }

    /// Set the background color to use
    pub fn fill_clk(self, color: Color) -> Self {
        Self { fill_clk: Some(color), ..self }
    }

    /// Set the background color to use
    pub fn fill_hov(self, color: Color) -> Self {
        Self { fill_hov: Some(color), ..self }
    }

    /// Set icon to use
    pub fn icon_texture<T: Into<Option<Texture2D>>>(self, icon: T) -> Self {
        self.layout.alloc_prepend(ICON_ID, None);
        Self { icon: icon.into(), ..self }
    }

    /// Update icon layout properties
    pub fn icon_layout<F: FnOnce(Layout) -> Layout>(self, f: F) -> Self {
        self.layout.set_sub(f(self.layout.sub(ICON_ID).unwrap().clone()));
        self
    }

    /// Set font to use
    pub fn label_font(self, font: Option<&'static [u8]>) -> Self {
        Self { label_font: font, ..self }
    }

    /// Set font size to use for the button label
    /// * handles scaling for mobile
    pub fn label_size(self, size: f32) -> Self {
        Self { label_font_size: size, ..self }
    }

    /// Set font color to use
    pub fn label_color(self, color: Color) -> Self {
        Self { label_font_color: color, ..self }
    }

    /// Set label layout to use
    pub fn label_layout<F: FnOnce(Layout) -> Layout>(self, f: F) -> Self {
        self.layout.set_sub(f(self.layout.sub(LABEL_ID).unwrap().clone()));
        self
    }

    /// Set layout to use
    pub fn layout<F: FnOnce(Layout) -> Layout>(self, f: F) -> Self {
        Self { layout: f(self.layout), ..self }
    }

    /// Create a new button instance
    pub fn build<T: AsRef<str>>(&self, label: T) -> Button {
        let conf = self.clone().layout(|x| x.copy().id(label.as_ref()));
        Button {
            conf,
            dirty: true,
            skin: None,
            label: label.as_ref().to_string(),
            clicked: false,
            activated: false,
        }
    }
}

/// Button encapsulates and extends Macroquad's button
#[derive(Debug, Clone)]
pub struct Button {
    conf: ButtonBuilder, // button configuration
    dirty: bool,         // track if the widget needs styling and shape calculation updates
    skin: Option<Skin>,  // skin to use for the entry titles
    label: String,       // button label text value
    clicked: bool,       // track button clicked state
    activated: bool,     // track button activation i.e. odd clicks
}

// Constructors and builder functions
impl Button {
    /// Create a new button instance with an icon
    /// * the icon will be scaled to match the font size
    /// * `label` is the text to display as the button label
    /// * `icon` is a texture to be displayed as the button icon
    pub fn icon<T: AsRef<str>>(label: T, icon: Texture2D) -> Self {
        ButtonBuilder::icon(icon).build(label)
    }

    /// Set background image to use
    pub fn image<T: Into<Option<Image>>>(self, image: T) -> Self {
        Self { dirty: true, conf: self.conf.image(image), ..self }
    }

    /// Set background image to use
    pub fn image_clk<T: Into<Option<Image>>>(self, image: T) -> Self {
        Self { dirty: true, conf: self.conf.image_clk(image), ..self }
    }

    /// Set background image to use
    pub fn image_hov<T: Into<Option<Image>>>(self, image: T) -> Self {
        Self { dirty: true, conf: self.conf.image_hov(image), ..self }
    }

    /// Set the background color used for the button
    pub fn fill(self, color: Color) -> Self {
        Self { dirty: true, conf: self.conf.fill(color), ..self }
    }

    /// Set icon to use
    pub fn icon_texture<T: Into<Option<Texture2D>>>(self, icon: T) -> Self {
        Self { dirty: true, conf: self.conf.icon_texture(icon), ..self }
    }

    /// Update icon layout properties
    pub fn icon_layout<F: FnOnce(Layout) -> Layout>(self, f: F) -> Self {
        Self { dirty: true, conf: self.conf.icon_layout(f), ..self }
    }

    /// Set font to use
    pub fn label_font(self, font: Option<&'static [u8]>) -> Self {
        Self { dirty: true, conf: self.conf.label_font(font), ..self }
    }

    /// Set font size to use for the button label
    /// * handles scaling for mobile
    pub fn label_size(self, size: f32) -> Self {
        Self { dirty: true, conf: self.conf.label_size(size), ..self }
    }

    /// Set font color to use
    pub fn label_color(self, color: Color) -> Self {
        Self { dirty: true, conf: self.conf.label_color(color), ..self }
    }

    /// Set label layout to use
    pub fn label_layout<F: FnOnce(Layout) -> Layout>(self, f: F) -> Self {
        Self { dirty: true, conf: self.conf.label_layout(f), ..self }
    }

    /// Set layout to use
    pub fn layout<F: FnOnce(Layout) -> Layout>(self, f: F) -> Self {
        Self { dirty: true, conf: self.conf.layout(f), ..self }
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

    /// Returns the widgets current shape (pos, size)
    pub fn shape(&self) -> (Vec2, Vec2) {
        self.conf.layout.shape()
    }

    /// Make layout, styling and shape calculation updates in prepartion for showing
    fn ui(&mut self, ui: &mut Ui) {
        if !self.dirty {
            return;
        }
        // Create the label style
        let mut style =
            ui.style_builder().text_color(self.conf.label_font_color).font_size(self.conf.label_font_size as u16);
        if let Some(color) = self.conf.label_font_color_clk {
            style = style.text_color_clicked(color);
        }
        if let Some(color) = self.conf.label_font_color_hov {
            style = style.text_color_hovered(color);
        }
        if let Some(font) = self.conf.label_font {
            style = style.font(font).unwrap();
        }
        let label_style = style.build();

        // Create the button style
        style =
            ui.style_builder().color(self.conf.fill).color_clicked(self.conf.fill).color_hovered(self.conf.fill);
        if let Some(background) = &self.conf.image {
            style = style.background(background.clone());
        }
        if let Some(background) = &self.conf.image_clk {
            style = style.background_clicked(background.clone());
        }
        if let Some(background) = &self.conf.image_hov {
            style = style.background_hovered(background.clone());
        }
        if let Some(color) = &self.conf.fill_clk {
            style = style.color_clicked(*color);
        }
        if let Some(color) = &self.conf.fill_hov {
            style = style.color_hovered(*color);
        }
        let button_style = style.build();

        // Create the skin based on override styles
        let skin = Skin { button_style, label_style, ..ui.default_skin() };

        // Calculate and cache button component sizes to reduce compute time
        let label_size = text_size(ui, &skin, Some(&self.label));
        if let Some(_) = &self.conf.icon {
            self.conf.layout.set_sub_size(ICON_ID, vec2(label_size.y + 5.0, label_size.y + 5.0));
        }
        self.conf.layout.set_sub_size(LABEL_ID, label_size);
        self.conf.layout.update();

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
        if let Some(parent) = layout {
            parent.append(&self.conf.layout);
        }

        // Draw button
        let (pos, size) = self.conf.layout.shape();
        if widgets::Button::new("").size(size).position(pos).ui(ui) {
            self.activated = !self.activated;
            self.clicked = true;
        }

        // Draw icon
        if let Some(icon) = &self.conf.icon {
            let (icon_pos, icon_size) = self.conf.layout.sub_shape(ICON_ID).unwrap();
            widgets::Texture::new(*icon).size(icon_size.x, icon_size.y).position(icon_pos).ui(ui);
        }

        // Draw label
        let (label_pos, label_size) = self.conf.layout.sub_shape(LABEL_ID).unwrap();
        widgets::Label::new(self.label.as_str()).size(label_size).position(label_pos).ui(ui);

        ui.pop_skin();

        self.clicked
    }
}
