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

/// ButtonBuilder provides the ability to preserve widget configuration and be able to repeatedly
/// create new widget instances based on this configuration rather than have to configure each
/// individual widget instance.
#[derive(Debug, Clone)]
pub struct ButtonBuilder {
    layout: Layout, // layout for the button overall

    // Background properties
    background: Option<Image>,           // background image to use
    background_clk: Option<Image>,       // background image to use when clicked
    background_hov: Option<Image>,       // background image to use when hovered
    background_color: Color,             // background color
    background_color_clk: Option<Color>, // background color when clicked
    background_color_hov: Option<Color>, // background color when hovered

    // Font properties
    label_font: Option<&'static [u8]>,   // font to use for label
    label_font_size: f32,                // font size to use for label
    label_font_color: Color,             // font color to use for label
    label_font_color_clk: Option<Color>, // font color to use for label when clicked
    label_font_color_hov: Option<Color>, // font color to use for label when hovered
    label_layout: Layout,                // layout for the label

    // Icon properties
    icon: Option<Texture2D>, // optional icon to display
    icon_layout: Layout,     // layout for the icon
}

impl ButtonBuilder {
    /// Create a new builder instance
    pub fn new() -> Self {
        Self {
            layout: Layout::default(),
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
            label_layout: Layout::default(),
            icon: None,
            icon_layout: Layout::default(),
        }
    }

    /// Set background image to use
    pub fn background<T: Into<Option<Image>>>(self, image: T) -> Self {
        Self { background: image.into(), ..self }
    }

    /// Set background image to use
    pub fn background_clk<T: Into<Option<Image>>>(self, image: T) -> Self {
        Self { background_clk: image.into(), ..self }
    }

    /// Set background image to use
    pub fn background_hov<T: Into<Option<Image>>>(self, image: T) -> Self {
        Self { background_hov: image.into(), ..self }
    }

    /// Set the background color used for the button
    pub fn background_color(self, color: Color) -> Self {
        Self { background_color: color, ..self }
    }

    /// Set font to use
    pub fn label_font(self, font: Option<&'static [u8]>) -> Self {
        Self { label_font: font, ..self }
    }

    /// Set font size to use for the button label
    /// * handles scaling for mobile
    pub fn label_font_size(self, size: f32) -> Self {
        Self { label_font_size: scale(size), ..self }
    }

    /// Set font color to use
    pub fn label_font_color(self, color: Color) -> Self {
        Self { label_font_color: color, ..self }
    }

    /// Set icon to use
    pub fn icon<T: Into<Option<Texture2D>>>(self, icon: T) -> Self {
        Self { icon: icon.into(), ..self }
    }

    /// Create a new widget instance from this builder
    pub fn build<T: AsRef<str>>(&self, label: T) -> Button {
        Button {
            dirty: true,
            skin: None,
            conf: self.clone(),
            label: label.as_ref().to_string(),
            clicked: false,
            activated: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Button {
    dirty: bool,         // track if a skin update is needed
    skin: Option<Skin>,  // skin to use for the entry titles
    conf: ButtonBuilder, // configuration of the button
    label: String,       // button label text value
    clicked: bool,       // track button clicked state
    activated: bool,     // track button activation i.e. odd clicks
}

/// Button encapsulates and extends Macroquad's button
impl Button {
    /// Create a new standard button instance
    pub fn new<T: AsRef<str>>(label: T) -> Self {
        ButtonBuilder::new().build(label)
    }

    /// Create a new button instance with an icon
    pub fn icon<T: AsRef<str>>(label: T, icon: Texture2D) -> Self {
        Button::new(label)
            .with_icon(icon)
            .with_icon_layout(Layout::new().with_margin(20., 10., 10., 10.))
            .with_label_layout(Layout::new().with_margin(10., 20., 10., 10.))
    }

    /// Set background image to use
    pub fn with_background<T: Into<Option<Image>>>(self, image: T) -> Self {
        Button { dirty: true, conf: ButtonBuilder { background: image.into(), ..self.conf }, ..self }
    }

    /// Set background image to use
    pub fn with_background_clk<T: Into<Option<Image>>>(self, image: T) -> Self {
        Button { dirty: true, conf: ButtonBuilder { background_clk: image.into(), ..self.conf }, ..self }
    }

    /// Set background image to use
    pub fn with_background_hov<T: Into<Option<Image>>>(self, image: T) -> Self {
        Button { dirty: true, conf: ButtonBuilder { background_hov: image.into(), ..self.conf }, ..self }
    }

    /// Set the background color used for the button
    pub fn with_background_color(self, color: Color) -> Self {
        Button { dirty: true, conf: ButtonBuilder { background_color: color, ..self.conf }, ..self }
    }

    /// Set font to use
    pub fn with_label_font(self, font: Option<&'static [u8]>) -> Self {
        Button { dirty: true, conf: ButtonBuilder { label_font: font, ..self.conf }, ..self }
    }

    /// Set font size to use for the button label
    /// * handles scaling for mobile
    pub fn with_label_font_size(self, size: f32) -> Self {
        Button { dirty: true, conf: ButtonBuilder { label_font_size: scale(size), ..self.conf }, ..self }
    }

    /// Set font color to use
    pub fn with_label_font_color(self, color: Color) -> Self {
        Button { dirty: true, conf: ButtonBuilder { label_font_color: color, ..self.conf }, ..self }
    }

    /// Set label layout to use
    pub fn with_label_layout(self, layout: Layout) -> Self {
        Button { conf: ButtonBuilder { label_layout: layout, ..self.conf }, ..self }
    }

    /// Set icon to use
    pub fn with_icon<T: Into<Option<Texture2D>>>(self, icon: T) -> Self {
        Button { conf: ButtonBuilder { icon: icon.into(), ..self.conf }, ..self }
    }

    /// Set icon layout to use
    pub fn with_icon_layout(self, layout: Layout) -> Self {
        Button { conf: ButtonBuilder { icon_layout: layout, ..self.conf }, ..self }
    }

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
        style = ui
            .style_builder()
            .color(self.conf.background_color)
            .color_clicked(self.conf.background_color)
            .color_hovered(self.conf.background_color);
        if let Some(background) = &self.conf.background {
            style = style.background(background.clone());
        }
        if let Some(background) = &self.conf.background_clk {
            style = style.background_clicked(background.clone());
        }
        if let Some(background) = &self.conf.background_hov {
            style = style.background_hovered(background.clone());
        }
        if let Some(color) = &self.conf.background_color_clk {
            style = style.color_clicked(*color);
        }
        if let Some(color) = &self.conf.background_color_hov {
            style = style.color_hovered(*color);
        }
        let button_style = style.build();

        // Create the skin based on override styles
        let skin = Skin { button_style, label_style, ..ui.default_skin() };

        // Calculate and cache button component sizes to reduce compute time
        self.conf.layout.reset();
        self.conf.icon_layout.reset();
        self.conf.label_layout.reset();
        let label_size = text_size(ui, &skin, Some(&self.label));
        if let Some(_) = &self.conf.icon {
            self.conf.icon_layout.alloc(ICON_ID, vec2(label_size.y, label_size.y));
            self.conf.layout.alloc(ICON_ID, self.conf.icon_layout.get_size());
        }
        self.conf.label_layout.alloc(LABEL_ID, label_size);
        self.conf.layout.alloc(LABEL_ID, self.conf.label_layout.get_size());

        self.skin = Some(skin);
        self.dirty = false;
    }

    /// Draw the widget on the screen
    /// * `layout` provides assistance and directives for sizing and positioning the widget
    /// * returns true when clicked in the current frame
    pub fn show(&mut self, ui: &mut Ui, layout: &mut Layout) -> bool {
        self.ui(ui);
        ui.push_skin(self.skin.as_ref().unwrap());
        self.clicked = false; // reset clicked

        // Draw button
        let (pos, size) = layout.alloc(&self.label, self.conf.layout.get_size());
        self.conf.layout.set_pos(pos.x, pos.y);
        if widgets::Button::new("").size(size).position(pos).ui(ui) {
            self.activated = !self.activated;
            self.clicked = true;
        }

        // Draw icon
        if let Some(icon) = &self.conf.icon {
            self.conf.icon_layout.set_pos(pos.x, pos.y);
            let (icon_pos, icon_size) = self.conf.icon_layout.pos_size_of(ICON_ID).unwrap();
            //let icon_pos = self.conf.icon_position.relative(icon_size, btn_size, Some(btn_pos));
            widgets::Texture::new(*icon).size(icon_size.x, icon_size.y).position(icon_pos).ui(ui);
        }

        // Draw label
        //let mut label_pos = self.conf.label_position.relative(self.label_size_calc, btn_size, Some(btn_pos));
        let (label_pos, label_size) = self.conf.layout.pos_size_of(LABEL_ID).unwrap();
        widgets::Label::new(self.label.as_str()).size(label_size).position(label_pos).ui(ui);

        ui.pop_skin();

        self.clicked
    }
}
