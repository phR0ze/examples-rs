//! Group provides a wrapper around the stock Macroquad group providing addtional functionality
//! * simpler more direct property manipulation
//! * relative positioning that adjusts for app sizing adjustments unlinke stock MQ Window widget
//! * background color and background image support
//! * padding support
//! * disable scrolling easily
use core::prelude::*;

/// GroupBuilder is the Group configuration split out of the Group struct's composition.  This
/// separating out of the configuration while composing from them allows for directly configuring
/// the Group or to configure a GroupBuilder and repeatedly use it to build new Group instances.
#[derive(Debug, Clone)]
pub struct GroupBuilder {
    size: Size,                    // size of the group on the screen
    position: Position,            // position the group on the screen
    padding: RectOffset,           // pad inside group pushing content in from edges
    background: Option<Texture2D>, // optional background image to use, takes priority over background color
    background_color: Color,       // background color to use if background is not set
    border_color: Option<Color>,   // optional border color to use
    scrolling: bool,               // enable scrolling when true
}

impl GroupBuilder {
    /// Instantiate a new instance
    pub fn new() -> GroupBuilder {
        GroupBuilder {
            size: Size::default(),
            position: Position::default(),
            padding: RectOffset::default(),
            background: None,
            background_color: colors::GRAY,
            border_color: None,
            scrolling: false,
        }
    }

    /// Set size of the group
    /// * handles scaling for mobile
    pub fn size(self, size: Size) -> Self {
        GroupBuilder { size, ..self }
    }

    /// Set position on the screen
    pub fn position<T: Into<Position>>(self, pos: T) -> Self {
        GroupBuilder { position: pos.into(), ..self }
    }

    /// Pad inside the group pushing content in from edges
    /// * handles scaling for mobile
    pub fn padding(self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        GroupBuilder { padding: scale_rect(left, right, top, bottom), ..self }
    }

    /// Set the background image to use. Takes priority over background color
    pub fn background(self, background: Texture2D) -> Self {
        GroupBuilder { background: Some(background), ..self }
    }

    /// Set the background color to use. Only has affect if background image not set
    pub fn background_color(self, color: Color) -> Self {
        GroupBuilder { background_color: color, ..self }
    }

    /// Set the border color to use
    pub fn border_color(self, color: Color) -> Self {
        GroupBuilder { border_color: Some(color), ..self }
    }

    /// Set scrolling state
    pub fn scrolling(self, scrolling: bool) -> Self {
        GroupBuilder { scrolling, ..self }
    }

    /// Instantiate a group object from the group configuration
    pub fn build<T: AsRef<str>>(&self, id: T) -> Group {
        Group {
            id: id.as_ref().to_string(),
            dirty: true,
            conf: self.clone(),
            skin: None,
            clicked: false,
            toggle: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Group {
    id: String,         // unique group id
    dirty: bool,        // track if the group needs updated before drawing
    conf: GroupBuilder, // track the group's configuration
    skin: Option<Skin>, // cached MQ skin for drawing
    clicked: bool,      // capture the click state
    toggle: bool,       // capture the toggle state i.e. true over odd click
}

impl Group {
    /// Create a new group instance
    pub fn new<T: AsRef<str>>(id: T) -> Self {
        Group {
            id: id.as_ref().to_string(),
            dirty: true,
            conf: GroupBuilder::new(),
            skin: None,
            clicked: false,
            toggle: false,
        }
    }

    /// Set size of the group
    /// * handles scaling for mobile
    pub fn with_size(self, size: Size) -> Self {
        Group { conf: GroupBuilder { size, ..self.conf }, ..self }
    }

    /// Set position on the screen
    pub fn with_position<T: Into<Position>>(self, pos: T) -> Self {
        Group { conf: GroupBuilder { position: pos.into(), ..self.conf }, ..self }
    }

    /// Pad inside the group pushing content in from edges
    /// * handles scaling for mobile
    pub fn with_padding(self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Group { conf: GroupBuilder { padding: scale_rect(left, right, top, bottom), ..self.conf }, ..self }
    }

    /// Set the background image to use. Takes priority over background color
    pub fn with_background(self, background: Texture2D) -> Self {
        Group { dirty: true, conf: GroupBuilder { background: Some(background), ..self.conf }, ..self }
    }

    /// Set the background color to use. Only has affect if background image not set
    pub fn with_background_color(self, color: Color) -> Self {
        Group { dirty: true, conf: GroupBuilder { background_color: color, ..self.conf }, ..self }
    }

    /// Set the border color to use
    pub fn with_border_color(self, color: Color) -> Self {
        Group { dirty: true, conf: GroupBuilder { border_color: Some(color), ..self.conf }, ..self }
    }

    /// Set scrolling state
    pub fn with_scrolling(self, scrolling: bool) -> Self {
        Group { dirty: true, conf: GroupBuilder { scrolling, ..self.conf }, ..self }
    }

    /// Set the background color to use. Only has affect if background image not set
    pub fn background_color(&mut self, color: Color) {
        self.dirty = true;
        self.conf.background_color = color;
    }

    /// Return the clicked state
    pub fn clicked(&self) -> bool {
        self.clicked
    }

    /// Return the toggle state i.e. every odd click will return true here while the even
    /// click will set this back to false.
    pub fn toggle(&self) -> bool {
        self.toggle
    }

    /// Update the macroquad skin based on the group's current properties
    fn update_skin(&mut self, ui: &mut Ui) {
        if !self.dirty {
            return;
        }

        // BLANK color gets rid of the default group 1px white border for group
        let border_color = self.conf.border_color.unwrap_or(BLANK);
        let group_style = ui.style_builder().color(border_color).color_hovered(BLANK).color_clicked(BLANK).build();

        // Configure the button background color if a background was not given
        let button_style = if self.conf.background.is_none() {
            ui.style_builder()
                .color(self.conf.background_color)
                .color_hovered(self.conf.background_color)
                .color_clicked(self.conf.background_color)
                .build()
        } else {
            ui.default_skin().button_style
        };

        // Hide the group scrollbar when content expands beyond the group size
        if !self.conf.scrolling {
            let scroll_width = 0.0;
            let scroll_multiplier = 0.0;
            let scrollbar_style =
                ui.style_builder().color(BLANK).color_hovered(BLANK).color_clicked(BLANK).build();
            let scrollbar_handle_style =
                ui.style_builder().color(BLANK).color_hovered(BLANK).color_clicked(BLANK).build();

            self.skin = Some(Skin {
                group_style,
                button_style,
                scrollbar_style,
                scrollbar_handle_style,
                scroll_width,
                scroll_multiplier,
                ..ui.default_skin()
            });
        } else {
            self.skin = Some(Skin { group_style, ..ui.default_skin() });
        }
        self.dirty = false;
    }

    /// Draw the widget and execute the callback with group properties
    /// * `cont_size` is the containing widget's size
    /// * `f` is a callback with params (Ui, cont_size, pos_offset)
    pub fn ui<F: FnOnce(&mut Ui, Vec2, Vec2)>(&mut self, ui: &mut Ui, cont_size: Vec2, f: F) {
        self.update_skin(ui);
        ui.push_skin(self.skin.as_ref().unwrap());

        // Using a outer containing group for all components for moveability
        let outer_size = self.conf.size.relative(cont_size);
        let outer_pos = self.conf.position.relative(outer_size, cont_size, None);
        widgets::Group::new(hash!(&self.id), outer_size).position(outer_pos).ui(ui, |ui| {
            // Draw button filling the entire group for clickability and background color
            if widgets::Button::new("").size(outer_size).position(vec2(0., 0.)).ui(ui) {
                self.clicked = true;
                self.toggle = !self.toggle;
            };
            ui.pop_skin();

            // Draw texture filling the entire group for background images. Texture2d grid was
            // 50x faster than a Button Image based grid of the same size
            if self.conf.background.is_some() {
                widgets::Texture::new(*self.conf.background.as_ref().unwrap())
                    .size(outer_size.x, outer_size.y)
                    .position(vec2(0., 0.))
                    .ui(ui);
            }

            // Draw content, passing inner size and positional offset
            let inner_size = vec2(
                outer_size.x - self.conf.padding.left - self.conf.padding.right,
                outer_size.y - self.conf.padding.top - self.conf.padding.bottom,
            );
            let inner_pos_offset = vec2(self.conf.padding.left, self.conf.padding.top);
            f(ui, inner_size, inner_pos_offset)
        });
    }
}
