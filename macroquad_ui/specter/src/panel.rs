//! Panel provides fundamental capabilities most other widgets are derived from.
//!
//! ## Features
//! * Layout management with nested child widget layout activation
//! * Frame support for background images, colors and border manipulation
//! * Capability to listen for user click interactions
//! * Uses the background image's size when no size is calculated
//!
//! ### Background images
//! Background images can be set via the underlying `Frame` which if no size is calculated for the
//! panel will be used as the size of the panel. If a size is calculated based on content or
//! manually set then that size will be used instead.
//!
//! ### Interactive
//! Listening for user click interactions can be manually manipulated with the interact functions
//! or if any interactive options are selected and interact hasn't been manually disabled it will be
//! automatically enabled. For example if a image is set for the click or hovered options and
//! interact hasn't been explicitely disabled then it will be automatically enabled. The same is
//! true for the color clicked and hovered options.
use std::ops::Index;

use crate::prelude::*;

pub struct Panel {
    frame: Frame,                  // frame properties
    layout: Layout,                // layout properties
    activated: bool,               // track clicks when in interactive mode
    interact: Option<bool>,        // enable/disable click intention activation
    interact_auto: bool,           // enable/disable click intention activation automatically
    widgets: Vec<Box<dyn Widget>>, // widgets to draw
}

impl Default for Panel {
    fn default() -> Self {
        Self {
            frame: Frame::new(),
            layout: Layout::new(""),
            activated: false,
            interact: None,
            interact_auto: false,
            widgets: vec![],
        }
    }
}

impl Clone for Panel {
    fn clone(&self) -> Self {
        Self {
            frame: self.frame.clone(),
            layout: self.layout.clone(),
            activated: self.activated,
            interact: self.interact,
            interact_auto: self.interact_auto,
            widgets: vec![],
        }
    }
}

// Constructors and builders
impl Panel {
    pub fn new<T: AsRef<str>>(id: T) -> Self {
        Self::default().id(id)
    }

    /// Create a new unique instance
    /// * similar to clone but allows for setting unique information
    /// * clones underlying components so there are no ties back to the orignal
    pub fn build<T: AsRef<str>>(&self, id: T) -> Self {
        self.clone().layout(|x| x.id(id))
    }

    /// Create a horizontal panel
    /// * lays out sub-layouts using the left to right packing mode
    pub fn horz<T: AsRef<str>>(id: T) -> Self {
        Self::new(id).layout(|x| x.mode(Mode::LeftToRight))
    }

    /// Create a vertical layout
    /// * lays out sub-layouts using the top to bottom packing mode
    pub fn vert<T: AsRef<str>>(id: T) -> Self {
        Self::new(id).layout(|x| x.mode(Mode::TopToBottom))
    }

    /// Set the widget's id
    pub fn id<T: AsRef<str>>(self, id: T) -> Self {
        Self {
            layout: self.layout.id(id.as_ref()),
            ..self
        }
    }

    /// Set the widget's frame properties
    pub fn frame(self, f: impl FnOnce(Frame) -> Frame) -> Self {
        let frame = f(self.frame);
        Self {
            interact_auto: frame.interact,
            frame,
            ..self
        }
    }

    /// Enable listening for click interactions
    pub fn interact(self) -> Self {
        Self {
            interact: Some(true),
            ..self
        }
    }

    /// Disable listening for click interactions
    pub fn no_interact(self) -> Self {
        Self {
            interact: Some(false),
            ..self
        }
    }

    /// Set the widget's layout properties
    pub fn layout(self, f: impl FnOnce(Layout) -> Layout) -> Self {
        Self {
            layout: f(self.layout),
            ..self
        }
    }
}

// Utility functions
impl Panel {
    /// Returns true if this widget has been clicked to toggle it on
    /// * only works when the widget is persisted outside the main loop
    pub fn activated(&self) -> bool {
        self.activated
    }

    /// Returns true if the panel was configured to be interactive or not
    /// disabled and automatically determined to be interactive.
    pub fn interactive(&self) -> bool {
        if let Some(interact) = self.interact {
            interact
        } else {
            self.interact_auto
        }
    }

    /// Draw the widget on the screen
    /// * `ui` is the Macroquad Ui engine
    pub fn ui(&mut self, ui: &mut Ui) -> Response {
        let mut context = ui.get_active_window_context();

        // Get panel position and size
        let (pos, mut size) = self.layout.shape();

        // If panel size is zero use the background image's size if it exists
        if size == Vec2::default() {
            if let Some(image) = self.frame.image {
                size = vec2(image.width(), image.height());
            }
        }

        // Register click intention for the panel
        let (hovered, clicked, mouse_down) = if self.interactive() {
            let rect = Rect::new(pos.x, pos.y, size.x as f32, size.y as f32);
            let (hovered, clicked) = context.register_click_intention(rect);
            if clicked {
                self.activated = !self.activated;
            }
            (hovered, clicked, context.input.is_mouse_down())
        } else {
            (false, false, false)
        };

        // Draw background
        let mut drawn = true;
        if hovered && mouse_down {
            // Draw clicked background image or color
            if let Some(image) = self.frame.image_clk {
                widgets::Texture::new(image).size(size.x, size.y).position(pos).ui(ui);
            } else if let Some(color) = self.frame.fill_clk {
                draw_rectangle(pos.x, pos.y, size.x, size.y, color);
            } else {
                drawn = false;
            }
        } else if hovered {
            // Draw hovered but not clicked background image or color
            if let Some(image) = self.frame.image_hov {
                widgets::Texture::new(image).size(size.x, size.y).position(pos).ui(ui);
            } else if let Some(color) = self.frame.fill_hov {
                draw_rectangle(pos.x, pos.y, size.x, size.y, color);
            } else {
                drawn = false;
            }
        }
        if !hovered || !drawn {
            // Draw normal background image or color
            if let Some(image) = self.frame.image {
                widgets::Texture::new(image).size(size.x, size.y).position(pos).ui(ui);
            } else if let Some(color) = self.frame.fill {
                draw_rectangle(pos.x, pos.y, size.x, size.y, color);
            }
        }

        // Draw widgets
        let mut responses = vec![];
        for x in self.widgets.iter_mut() {
            responses.push(x.show_p(ui));
        }

        Response {
            id: self.layout.get_id(),
            activated: self.activated,
            clicked,
            hovered,
            mouse_down,
            items: responses,
        }
    }
}

// impl Index<Panel> for Panel {
//     type Output = dyn Widget;

//     fn index(&self, panel: Panel) -> &Self::Output {
//         &panel
//     }
// }

impl LayoutManager for Panel {
    /// Add the given widget to this widget's layout management
    /// * similar to `append` but consumes and returns self
    fn add(mut self, widget: impl Widget + 'static) -> Self {
        self.append(widget);
        self
    }

    /// Add the given widget to this widget's layout management
    fn append(&mut self, widget: impl Widget + 'static) {
        self.layout.append(&widget.get_layout());
        self.widgets.push(Box::new(widget));
    }

    /// Get a reference to the widget by id
    fn get<T: AsRef<str>>(&self, id: T) -> Option<&Box<dyn Widget>> {
        self.widgets.iter().find(|x| x.get_id() == id.as_ref().to_string())
    }
}

impl Widget for Panel {
    /// Cast the concreate type as an any
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Get widget's frame
    fn get_frame(&self) -> &Frame {
        &self.frame
    }

    /// Returns a reference clone to the Widget's layout
    fn get_layout(&self) -> Layout {
        self.layout.ptr()
    }

    /// Draw the widget on the screen
    /// * `ui` is the Macroquad Ui engine
    fn show_p(&mut self, ui: &mut Ui) -> Response {
        self.ui(ui)
    }
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn widgets_ref() {
        let vec = vec![Panel::new("0")];
        let foo = &vec[0];
    }
}
