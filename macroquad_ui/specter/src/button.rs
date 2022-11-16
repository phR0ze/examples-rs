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
    layout: Layout,           // layout
    image: Option<Image>,     // background image to use
    image_clk: Option<Image>, // background image to use when clicked
    image_hov: Option<Image>, // background image to use when hovered
    fill: Color,              // background color
    fill_clk: Option<Color>,  // background color when clicked
    fill_hov: Option<Color>,  // background color when hovered
    label: Label,             // label widget
    icon: Option<Texture2D>,  // optional icon to display
}

impl ButtonBuilder {
    /// Create a new builder instance
    pub fn new() -> Self {
        let layout = Layout::horz("");
        let label = Label::new("").layout(|_| layout.sub_alloc_append(LABEL_ID, None));

        Self {
            layout,
            image: None,
            image_clk: None,
            image_hov: None,
            fill: colors::BLANK,
            fill_clk: None,
            fill_hov: None,
            label,
            icon: None,
        }
    }

    /// Create a new button instance with an icon
    /// * the icon will be scaled to match the font size
    /// * `icon` is a texture to be displayed as the button icon
    pub fn icon(icon: Texture2D) -> Self {
        ButtonBuilder::new()
            .icon_texture(icon)
            .icon_layout(|x| x.align(Align::Center).margins(10., 10., 5., 5.))
            .label_layout(|x| x.align(Align::Center).margins(10., 20., 0., 0.))
    }

    /// Set background image to use
    pub fn image<T: Into<Option<Image>>>(self, image: T) -> Self {
        Self {
            image: image.into(),
            ..self
        }
    }

    /// Set background image to use
    pub fn image_clk<T: Into<Option<Image>>>(self, image: T) -> Self {
        Self {
            image_clk: image.into(),
            ..self
        }
    }

    /// Set background image to use
    pub fn image_hov<T: Into<Option<Image>>>(self, image: T) -> Self {
        Self {
            image_hov: image.into(),
            ..self
        }
    }

    /// Set the background color used for the button
    pub fn fill(self, color: Color) -> Self {
        Self {
            fill: color,
            ..self
        }
    }

    /// Set the background color to use
    pub fn fill_clk(self, color: Color) -> Self {
        Self {
            fill_clk: Some(color),
            ..self
        }
    }

    /// Set the background color to use
    pub fn fill_hov(self, color: Color) -> Self {
        Self {
            fill_hov: Some(color),
            ..self
        }
    }

    /// Set icon to use
    pub fn icon_texture<T: Into<Option<Texture2D>>>(self, icon: T) -> Self {
        self.layout.sub_alloc_prepend(ICON_ID, None);
        Self {
            icon: icon.into(),
            ..self
        }
    }

    /// Update icon layout properties
    pub fn icon_layout<F: FnOnce(Layout) -> Layout>(self, f: F) -> Self {
        let sub = f(self.layout.sub(ICON_ID).unwrap().ptr());
        self.layout.subs_update(&sub);
        self
    }

    /// Set font to use
    pub fn label_font(self, font: Option<&'static [u8]>) -> Self {
        Self {
            label: self.label.font(font),
            ..self
        }
    }

    /// Set font size to use for the button label
    /// * handles scaling for mobile
    pub fn label_size(self, size: f32) -> Self {
        Self {
            label: self.label.size(size),
            ..self
        }
    }

    /// Set font color to use
    pub fn label_color(self, color: Color) -> Self {
        Self {
            label: self.label.color(color),
            ..self
        }
    }

    /// Set label layout to use
    pub fn label_layout<F: FnOnce(Layout) -> Layout>(self, f: F) -> Self {
        Self {
            label: self.label.layout(f),
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
    pub fn build<T: AsRef<str>>(&self, label: T) -> Button {
        let mut conf = self.clone().layout(|x| x.id(label.as_ref()));
        conf.label.set_text(label.as_ref());
        Button {
            conf,
            dirty: true,
            skin: None,
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
        Self {
            dirty: true,
            conf: self.conf.image(image),
            ..self
        }
    }

    /// Set background image to use
    pub fn image_clk<T: Into<Option<Image>>>(self, image: T) -> Self {
        Self {
            dirty: true,
            conf: self.conf.image_clk(image),
            ..self
        }
    }

    /// Set background image to use
    pub fn image_hov<T: Into<Option<Image>>>(self, image: T) -> Self {
        Self {
            dirty: true,
            conf: self.conf.image_hov(image),
            ..self
        }
    }

    /// Set the background color used for the button
    pub fn fill(self, color: Color) -> Self {
        Self {
            dirty: true,
            conf: self.conf.fill(color),
            ..self
        }
    }

    /// Set icon to use
    pub fn icon_texture<T: Into<Option<Texture2D>>>(self, icon: T) -> Self {
        Self {
            dirty: true,
            conf: self.conf.icon_texture(icon),
            ..self
        }
    }

    /// Update icon layout properties
    pub fn icon_layout<F: FnOnce(Layout) -> Layout>(self, f: F) -> Self {
        Self {
            dirty: true,
            conf: self.conf.icon_layout(f),
            ..self
        }
    }

    /// Set font to use
    pub fn label_font(self, font: Option<&'static [u8]>) -> Self {
        Self {
            dirty: true,
            conf: self.conf.label_font(font),
            ..self
        }
    }

    /// Set font size to use for the button label
    /// * handles scaling for mobile
    pub fn label_size(self, size: f32) -> Self {
        Self {
            dirty: true,
            conf: self.conf.label_size(size),
            ..self
        }
    }

    /// Set font color to use
    pub fn label_color(self, color: Color) -> Self {
        Self {
            dirty: true,
            conf: self.conf.label_color(color),
            ..self
        }
    }

    /// Set label layout to use
    pub fn label_layout<F: FnOnce(Layout) -> Layout>(self, f: F) -> Self {
        Self {
            dirty: true,
            conf: self.conf.label_layout(f),
            ..self
        }
    }

    /// Set layout to use
    pub fn layout<F: FnOnce(Layout) -> Layout>(self, f: F) -> Self {
        Self {
            dirty: true,
            conf: self.conf.layout(f),
            ..self
        }
    }
}

// Helper functions
impl Button {
    /// Button label
    pub fn label(&self) -> &str {
        self.conf.label.get_text()
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
    /// * Note: will be called automatically in most cases. Only useful to call when composing
    /// other widgets from this widget
    pub fn ui(&mut self, ui: &mut Ui) {
        if !self.dirty {
            return;
        }

        // Create the label style
        self.conf.label.ui(ui);

        // Create the button style
        let mut style =
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
        let skin = Skin {
            button_style,
            ..ui.default_skin()
        };

        // Calculate and cache button component sizes to reduce compute time
        let (_, label_size) = self.conf.label.shape();
        if let Some(_) = &self.conf.icon {
            self.conf.layout.sub_set_size(ICON_ID, label_size.y + 5.0, label_size.y + 5.0);
        }

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
            parent.subs_append(&self.conf.layout);
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
        self.conf.label.show(ui, None);

        ui.pop_skin();

        self.clicked
    }
}
