//! Panel provides fundamental capabilities most other widgets are derived from.
//!
//! ## Features
//! * Layout management with nested child widget layout activation
//! * Frame support for background images, colors and border manipulation
//! * Capability to listen for user click interactions
//!
//! ### Interactive
//! Listening for user click interactions can be manually manipulated with the interact functions
//! or if any interactive options are selected and interact hasn't been manually disabled it will be
//! automatically enabled.
use crate::prelude::*;

pub struct Panel {
    frame: Frame,                  // frame properties
    layout: Layout,                // layout properties
    interact: Option<bool>,        // enable/disable click intention activation
    interact_auto: bool,           // enable/disable click intention activation automatically
    widgets: Vec<Box<dyn Widget>>, // widgets to draw
}

/// Default implementation for Panel
impl Default for Panel {
    fn default() -> Self {
        Self {
            frame: Frame::new(),
            layout: Layout::new(""),
            interact: None,
            interact_auto: false,
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
        let (pos, size) = self.layout.shape();

        // Register click intention for the panel
        let (hovered, clicked, mouse_down) = if self.interactive() {
            let rect = Rect::new(pos.x, pos.y, size.x as f32, size.y as f32);
            let (hovered, clicked) = context.register_click_intention(rect);
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
            clicked,
            hovered,
            mouse_down,
            responses,
        }
    }
}

impl Clone for Panel {
    fn clone(&self) -> Self {
        Self {
            frame: self.frame.clone(),
            layout: self.layout.clone(),
            interact: self.interact,
            interact_auto: self.interact_auto,
            widgets: vec![],
        }
    }
}

impl LayoutManager for Panel {
    /// Add the given widget to this widget's layout management
    /// * similar to `append` but consumes and returns self
    fn add(mut self, widget: impl Widget + 'static) -> Self {
        self.append(widget);
        self
    }

    /// Add the given widget to this widget's layout management
    fn append(&mut self, widget: impl Widget + 'static) {
        self.layout.append(&widget.layout_ptr());
        self.widgets.push(Box::new(widget));
    }
}

impl Widget for Panel {
    /// Returns a reference clone to the Widget's layout
    fn layout_ptr(&self) -> Layout {
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
    fn test_foo() {
        assert_eq!(vec2(2., 2.), vec2(2., 2.));
    }
}
