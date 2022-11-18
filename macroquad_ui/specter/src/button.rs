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
    clicked: bool,        // track button clicked state
    activated: bool,      // track button activation i.e. odd clicks
    panel: Panel,         // underlying panel
    label: Label,         // label widget
    image: Option<Image>, // optional icon to display
}

impl Default for Button {
    fn default() -> Self {
        let label = Label::default().layout(|x| x.id(LABEL_ID).align(Align::Center).margins(5., 10., 6., 5.));
        let panel = Panel::default().layout(|x| x.mode(Mode::LeftToRight)).interact();
        panel.layout_ptr().append(&label.layout_ptr());

        Self {
            clicked: false,
            activated: false,
            panel,
            label,
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
        let image =
            self.image.unwrap_or_default().layout(|x| x.id(ICON_ID).align(Align::Center).margins(10., 5., 0., 0.));
        self.panel.layout_ptr().prepend(&image.layout_ptr());

        Self {
            image: Some(f(image)),
            ..self
        }
    }

    /// Set the widget's label properties
    pub fn label<F: FnOnce(Label) -> Label>(self, f: F) -> Self {
        Self {
            label: f(self.label),
            ..self
        }
    }

    /// Set the widget's layout properties
    pub fn layout<F: FnOnce(Layout) -> Layout>(self, f: F) -> Self {
        Self {
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
    pub fn ui(&mut self, ui: &mut Ui) -> Response {
        self.clicked = false; // reset clicked

        // Make pre-calculations which will impact panel size
        let size = self.label.pre_calc(ui);
        if let Some(image) = &self.image {
            image.layout_ptr().set_size(size.y, size.y);
        }

        // Draw button panel
        let response = self.panel.ui(ui);
        if response.clicked {
            self.activated = !self.activated;
            self.clicked = true;
        }

        // Draw label
        self.label.show_p(ui);

        // Draw icon
        if let Some(image) = &mut self.image {
            image.show_p(ui);
        }

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
