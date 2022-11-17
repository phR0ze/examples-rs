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

#[derive(Debug, Clone)]
pub struct PanelBuilder {
    frame: Frame,           // frame properties
    layout: Layout,         // layout properties
    interact: Option<bool>, // enable/disable click intention activation
    interact_auto: bool,    // enable/disable click intention activation automatically
}

impl PanelBuilder {
    pub fn new() -> Self {
        Self {
            frame: Frame::new(),
            layout: Layout::new(""),
            interact: None,
            interact_auto: false,
        }
    }

    /// Set the frame's properties
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

    /// Set layout to use
    pub fn layout(self, f: impl FnOnce(Layout) -> Layout) -> Self {
        Self {
            layout: f(self.layout),
            ..self
        }
    }

    /// Create a new Panel instance
    pub fn build<T: AsRef<str>>(&self, id: T) -> Panel {
        Panel {
            conf: Self {
                frame: self.frame.clone(),
                layout: self.layout.clone().id(id.as_ref()),
                interact: self.interact,
                interact_auto: self.interact_auto,
            },
            widgets: vec![],
        }
    }
}

pub struct Panel {
    conf: PanelBuilder,            // configuration
    widgets: Vec<Box<dyn Widget>>, // widgets to draw
}

// Constructors and builders
impl Panel {
    pub fn new<T: AsRef<str>>(id: T) -> Self {
        PanelBuilder::new().build(id)
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

    /// Set the frame's properties
    pub fn frame(self, f: impl FnOnce(Frame) -> Frame) -> Self {
        Self {
            conf: self.conf.frame(f),
            ..self
        }
    }

    /// Enable listening for click interactions
    pub fn interact(self) -> Self {
        Self {
            conf: self.conf.interact(),
            ..self
        }
    }

    /// Disable listening for click interactions
    pub fn no_interact(self) -> Self {
        Self {
            conf: self.conf.no_interact(),
            ..self
        }
    }

    /// Set layout to use
    pub fn layout(self, f: impl FnOnce(Layout) -> Layout) -> Self {
        Self {
            conf: self.conf.layout(f),
            ..self
        }
    }
}

// Utility functions
impl Panel {
    /// Returns true if the panel was configured to be interactive or not
    /// disabled and automatically determined to be interactive.
    pub fn interactive(&self) -> bool {
        if let Some(interact) = self.conf.interact {
            interact
        } else {
            self.conf.interact_auto
        }
    }

    /// Draw the widget on the screen
    /// * `ui` is the Macroquad Ui engine
    fn ui(&mut self, ui: &mut Ui) -> Response {
        let mut context = ui.get_active_window_context();

        // Get panel position and size
        let (pos, size) = self.conf.layout.shape();

        // Register click intention for the panel
        let (hovered, clicked) = if self.interactive() {
            let rect = Rect::new(pos.x, pos.y, size.x as f32, size.y as f32);
            context.register_click_intention(rect)
        } else {
            (false, false)
        };

        // Draw background
        let mut drawn = true;
        if hovered && context.input.is_mouse_down() {
            // Draw clicked background image or color
            if let Some(image) = self.conf.frame.image_clk {
                widgets::Texture::new(image).size(size.x, size.y).position(pos).ui(ui);
            } else if let Some(color) = self.conf.frame.fill_clk {
                draw_rectangle(pos.x, pos.y, size.x, size.y, color);
            } else {
                drawn = false;
            }
        } else if hovered {
            // Draw hovered but not clicked background image or color
            if let Some(image) = self.conf.frame.image_hov {
                widgets::Texture::new(image).size(size.x, size.y).position(pos).ui(ui);
            } else if let Some(color) = self.conf.frame.fill_hov {
                draw_rectangle(pos.x, pos.y, size.x, size.y, color);
            } else {
                drawn = false;
            }
        }
        if !hovered || !drawn {
            // Draw normal background image or color
            if let Some(image) = self.conf.frame.image {
                widgets::Texture::new(image).size(size.x, size.y).position(pos).ui(ui);
            } else if let Some(color) = self.conf.frame.fill {
                draw_rectangle(pos.x, pos.y, size.x, size.y, color);
            }
        }

        // Draw widgets
        let mut responses = vec![];
        for x in self.widgets.iter_mut() {
            responses.push(x.show_p(ui));
        }

        Response {
            id: self.conf.layout.get_id(),
            clicked,
            hovered,
            responses,
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
        self.conf.layout.append(&widget.layout_ptr());
        self.widgets.push(Box::new(widget));
    }
}

impl Widget for Panel {
    /// Returns a reference clone to the Widget's layout
    fn layout_ptr(&self) -> Layout {
        self.conf.layout.ptr()
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
