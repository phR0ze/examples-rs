//! Button widget provides:
//! * Border color for regular, clicked and hovered states
//! * Label positioning and sizing inside the button
//! * Icon support with positioning and sizing inside button
//! * Button activated toggle
//! * Calculated sizing and positioning relative to containing widget
//! * Builder for reusable layout but also direct modification
use crate::prelude::*;

const ICON_ID: &'static str = "icon";
const LABEL_ID: &'static str = "label";

/// Button encapsulates and extends Macroquad's button
#[derive(Clone)]
pub struct Button {
    dirty: bool,          // track if the widget needs styling and shape calculation updates
    clicked: bool,        // track button clicked state
    activated: bool,      // track button activation i.e. odd clicks
    panel: Panel,         // underlying panel
    label: Label,         // label widget
    image: Option<Image>, // optional icon to display
}

impl Default for Button {
    fn default() -> Self {
        Self {
            dirty: true,
            clicked: false,
            activated: false,
            panel: Panel::default(),
            label: Label::default(),
            image: None,
        }
    }
}

// Constructors and builder functions
impl Button {
    /// Create a new button instance with an icon
    /// * the icon will be scaled to match the font size
    /// * `id` is the widget identifier
    /// * `text` is the text to display as the button label
    /// * `icon` is a texture to be displayed as the button icon
    pub fn icon<T: AsRef<str>>(id: T, text: T, icon: Texture2D) -> Self {
        Self::default().id(id).label(|x| x.text(text)).image(|x| x.image(icon))
    }

    /// Create a new unique instance
    /// * similar to clone but allows for setting unique information
    /// * clones underlying components so there are no ties back to the orignal
    pub fn build<T: AsRef<str>>(&self, id: T, text: T) -> Self {
        self.clone().id(id).label(|x| x.text(text))
    }

    /// Set the widget's frame properties
    pub fn frame(self, f: impl FnOnce(Frame) -> Frame) -> Self {
        Self {
            panel: self.panel.frame(f),
            ..self
        }
    }

    /// Set the widget's id
    pub fn id<T: AsRef<str>>(self, id: T) -> Self {
        Self {
            panel: self.panel.id(id),
            ..self
        }
    }

    /// Set the widget's image properties
    pub fn image<F: FnOnce(Image) -> Image>(self, f: F) -> Self {
        Self {
            dirty: true,
            image: Some(f(self.image.unwrap_or_default())),
            ..self
        }
    }

    /// Set the widget's label properties
    pub fn label<F: FnOnce(Label) -> Label>(self, f: F) -> Self {
        Self {
            dirty: true,
            label: f(self.label),
            ..self
        }
    }

    /// Set the widget's layout properties
    pub fn layout<F: FnOnce(Layout) -> Layout>(self, f: F) -> Self {
        Self {
            dirty: true,
            panel: self.panel.layout(f),
            ..self
        }
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

    /// Draw the widget on the screen
    /// * `ui` is the Macroquad Ui engine
    fn ui(&mut self, ui: &mut Ui) -> Response {
        self.clicked = false; // reset clicked

        // Draw button panel
        let response = self.panel.ui(ui);
        if response.clicked {
            self.activated = !self.activated;
            self.clicked = true;
        }

        // // Draw icon
        // if let Some(icon) = &self.conf.icon {
        //     let (icon_pos, icon_size) = self.conf.layout.sub_shape(ICON_ID).unwrap();
        //     widgets::Texture::new(*icon).size(icon_size.x, icon_size.y).position(icon_pos).ui(ui);
        // }

        // // Draw label
        // self.conf.label.show_p(ui);

        // self.clicked
        response
    }
}

impl Widget for Button {
    /// Returns a reference clone to the Widget's layout
    fn layout_ptr(&self) -> Layout {
        self.panel.layout_ptr()
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

    //use super::*;

    #[test]
    fn test() {
        //
    }
}
