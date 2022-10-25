//! Button encapsulates and extends Macroquad's button supporting:
//! * Border color for regular, clicked and hovered
//! * Icon support with positioning and sizing inside button
//!
//! * Button activated toggle
//! * Label positioning inside the button
//! * Calculated sizing and positioning relative to containing widget
use crate::prelude::*;

/// ButtonBuilder provides the ability to preserve widget configuration and be able to repeatedly
/// create new widget instances based on this configuration rather than have to configure each
/// individual widget instance.
#[derive(Debug, Clone)]
pub struct ButtonBuilder {
    layout: Layout,      // layout management
    size: Size,          // sizing of the widget
    position: Position,  // position of of the widget
    padding: RectOffset, // spaced provided around content size

    // Background properties
    background: Option<Image>,           // background image to use
    background_clk: Option<Image>,       // background image to use when clicked
    background_hov: Option<Image>,       // background image to use when hovered
    background_color: Color,             // background color
    background_color_clk: Option<Color>, // background color when clicked
    background_color_hov: Option<Color>, // background color when hovered

    // Font properties
    label_font: Option<&'static [u8]>,   // font to use for label
    label_position: Position,            // position of the label within the button
    label_font_size: f32,                // font size to use for label
    label_font_color: Color,             // font color to use for label
    label_font_color_clk: Option<Color>, // font color to use for label when clicked
    label_font_color_hov: Option<Color>, // font color to use for label when hovered

    // Icon properties
    icon: Option<Texture2D>, // optional icon to display
    icon_size: Size,         // size to use for icon when drawing
    icon_margin: RectOffset, // icon offset
    icon_position: Position, // position of the icon within the button
}

impl ButtonBuilder {
    /// Create a new builder instance
    pub fn new() -> Self {
        Self {
            layout: Layout::new().expand(),
            size: Size::default(),
            position: Position::default(),
            padding: RectOffset::default(),
            background: None,
            background_clk: None,
            background_hov: None,
            background_color: colors::BLANK,
            background_color_clk: None,
            background_color_hov: None,
            label_font: None,
            label_position: Position::default(),
            label_font_size: scale(DEFAULT_FONT_SIZE),
            label_font_color: colors::BLACK,
            label_font_color_clk: None,
            label_font_color_hov: None,
            icon: None,
            icon_size: Size::Dynamic,
            icon_margin: RectOffset::default(),
            icon_position: Position::LeftCenter(None),
        }
    }

    /// Set the button's size directive
    /// * handles scaling for mobile
    pub fn size(self, size: Size) -> Self {
        Self { size: size.scale(), ..self }
    }

    /// Position the button on the screen
    /// * handles scaling for mobile
    pub fn position(self, pos: Position) -> Self {
        Self { position: pos.scale(), ..self }
    }

    /// Pad inside widget pushing content in from edges
    /// * handles scaling for mobile
    pub fn padding(self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self { padding: scale_rect(left, right, top, bottom), ..self }
    }

    /// Pad inside widget pushing content in from edges
    /// * handles scaling for mobile
    pub fn padding_p(self, padding: RectOffset) -> Self {
        Self { padding: scale_rect_p(padding), ..self }
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

    /// Position the label inside the button
    /// * handles scaling for mobile
    pub fn label_position(self, pos: Position) -> Self {
        Self { label_position: pos, ..self }
    }

    /// Set icon to use
    pub fn icon<T: Into<Option<Texture2D>>>(self, icon: T) -> Self {
        Self { icon: icon.into(), ..self }
    }

    /// Set icon size to use
    pub fn icon_size(self, size: Size) -> Self {
        Self { icon_size: size, ..self }
    }

    /// Set icon margin to use
    pub fn icon_margin(self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self { icon_margin: RectOffset::new(left, right, top, bottom), ..self }
    }

    /// Set icon margin to use
    pub fn icon_margin_p(self, margin: RectOffset) -> Self {
        Self { icon_margin: margin, ..self }
    }

    /// Position the icon inside the button
    /// * handles scaling for mobile
    pub fn icon_position(self, pos: Position) -> Self {
        Self { icon_position: pos.scale(), ..self }
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
            label_size: Vec2::default(),
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
    label_size: Vec2,    // calculated size of the label
}

/// Button encapsulates and extends Macroquad's button
impl Button {
    /// Create a new standard button instance
    pub fn new<T: AsRef<str>>(label: T) -> Self {
        ButtonBuilder::new().build(label)
    }

    /// Create a new button instance with an icon
    pub fn icon<T: AsRef<str>>(label: T, icon: Texture2D) -> Self {
        let mut button = Button::new(label)
            .with_icon(icon)
            .with_icon_margin(20., 20., 0., 0.)
            .with_icon_position(Position::LeftCenter(None))
            .with_label_position(Position::LeftCenter(rect(80., 0., 3., 0.)));

        //.with_layout(Layout::new().)
        button
    }

    /// Set the button's layout manager
    pub fn with_layout(self, layout: Layout) -> Self {
        Button { dirty: true, conf: ButtonBuilder { layout, ..self.conf }, ..self }
    }

    /// Set the button's size directive
    /// * handles scaling for mobile
    pub fn with_size(self, size: Size) -> Self {
        Button { dirty: true, conf: ButtonBuilder { size: size.scale(), ..self.conf }, ..self }
    }

    /// Position the button on the screen
    /// * handles scaling for mobile
    pub fn with_position(self, pos: Position) -> Self {
        Button { conf: ButtonBuilder { position: pos.scale(), ..self.conf }, ..self }
    }

    /// Pad inside widget pushing content in from edges
    /// * handles scaling for mobile
    pub fn with_padding(self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Button { conf: ButtonBuilder { padding: scale_rect(left, right, top, bottom), ..self.conf }, ..self }
    }

    /// Pad inside widget pushing content in from edges
    /// * handles scaling for mobile
    pub fn with_padding_p(self, padding: RectOffset) -> Self {
        Button { conf: ButtonBuilder { padding: scale_rect_p(padding), ..self.conf }, ..self }
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

    /// Position the label inside the button
    /// * handles scaling for mobile
    pub fn with_label_position(self, pos: Position) -> Self {
        Button { conf: ButtonBuilder { label_position: pos, ..self.conf }, ..self }
    }

    /// Set icon to use
    pub fn with_icon<T: Into<Option<Texture2D>>>(self, icon: T) -> Self {
        Button { conf: ButtonBuilder { icon: icon.into(), ..self.conf }, ..self }
    }

    /// Set icon size to use
    pub fn with_icon_size(self, size: Size) -> Self {
        Button { conf: ButtonBuilder { icon_size: size, ..self.conf }, ..self }
    }

    /// Set icon margin to use
    pub fn with_icon_margin(self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Button {
            conf: ButtonBuilder { icon_margin: RectOffset::new(left, right, top, bottom), ..self.conf },
            ..self
        }
    }

    /// Set icon margin to use
    pub fn with_icon_margin_p(self, margin: RectOffset) -> Self {
        Button { conf: ButtonBuilder { icon_margin: margin, ..self.conf }, ..self }
    }

    /// Position the icon inside the button
    /// * handles scaling for mobile
    pub fn with_icon_position(self, pos: Position) -> Self {
        Button { conf: ButtonBuilder { icon_position: pos.scale(), ..self.conf }, ..self }
    }

    /// Button label
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns the position of the button
    /// * `container` is the containing widget's size to relatively position against
    /// * `offset` any positional offset to take into account
    pub fn position(&mut self, ui: &mut Ui, container: Vec2, offset: Option<Vec2>) -> Vec2 {
        let size = self.size(ui, container);
        self.conf.position.relative(size, container, offset)
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

    /// Calculate the size based on the size directive and the given container size
    /// * `container` is the containing widget's size to relatively size against
    // pub fn size(&mut self, ui: &mut Ui, container: Vec2) -> Vec2 {
    //     self.update(ui);

    //     // Calculate the relative size based on the containing widget's size
    //     let mut size = self.conf.size.relative(container, Some(self.label_size_calc));

    //     // Take padding into account
    //     size.x += self.conf.padding.left + self.conf.padding.right;
    //     size.y += self.conf.padding.top + self.conf.padding.bottom;

    //     // Take icon size into account
    //     if let Some(icon) = &self.conf.icon {
    //         let icon_size = self.conf.icon_size.relative(size, Some(vec2(icon.width(), icon.height())));
    //         size.x += icon_size.x + self.conf.icon_margin.left + self.conf.icon_margin.right;
    //         size.y +=
    //             icon_size.y + self.conf.icon_margin.top + self.conf.icon_margin.bottom - self.label_size_calc.y;
    //     }

    //     size
    // }

    /// Update the skin and other Ui required values in prepartion for `show`
    fn update(&mut self, ui: &mut Ui) {
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
        self.skin = Some(skin);

        // Calculate text size
        let mut layout = Layout::new().expand().padding_p(self.conf.padding);
        let label_size = text_size(ui, &skin, Some(&self.label));
        let icon_size = if let Some(icon) = &self.conf.icon {
            let mut icon_size = vec2(icon.width(), icon.height());
            icon_size.x += self.conf.icon_margin.left + self.conf.icon_margin.right;
            icon_size.y += self.conf.icon_margin.top + self.conf.icon_margin.bottom;
            icon_size
        } else {
            vec2(0., 0.)
        };
        layout.alloc(icon_size);
        layout.alloc(label_size);
        self.conf.layout = layout;

        self.dirty = false;
    }

    /// Draw the widget on the screen
    /// * `layout` provides layout directive support for
    /// * returns true when clicked in the current frame
    pub fn show(&mut self, ui: &mut Ui, layout: &mut Layout) -> bool {
        self.update(ui);
        ui.push_skin(self.skin.as_ref().unwrap());
        self.clicked = false; // reset clicked

        // Get layout allocation
        //let (pos, size) = layout.alloc(self.size);

        // // Draw button
        // let btn_size = self.size(ui, container);
        // let btn_pos = self.position(ui, container, offset);
        // if widgets::Button::new("").size(btn_size).position(btn_pos).ui(ui) {
        //     self.activated = !self.activated;
        //     self.clicked = true;
        // }

        // // Draw icon
        // if let Some(icon) = &self.conf.icon {
        //     let icon_size = vec2(icon.width(), icon.height());
        //     let icon_pos = self.conf.icon_position.relative(icon_size, btn_size, Some(btn_pos));
        //     widgets::Texture::new(*icon).size(icon_size.x, icon_size.y).position(icon_pos).ui(ui);

        //     // Update label position to start after icon margin
        // }

        // // Calculate label position
        // let mut label_pos = self.conf.label_position.relative(self.label_size_calc, btn_size, Some(btn_pos));

        // // Draw label
        // widgets::Label::new(self.label.as_str()).size(self.label_size_calc).position(label_pos).ui(ui);

        ui.pop_skin();

        self.clicked
    }
}
