//! Widgeet
//!
use crate::prelude::*;

pub trait Widget: Any {
    /// Cast the concreate type as an any
    fn as_any(&self) -> &dyn Any;

    /// Get widget id
    fn get_id(&self) -> String {
        self.get_layout().get_id()
    }

    /// Get widget's frame
    fn get_frame(&self) -> &Frame;

    /// Returns a reference clone to the Widget's layout
    fn get_layout(&self) -> Layout;

    /// Get the widget's shape as a (position, size) tuple
    fn shape(&self) -> (Vec2, Vec2) {
        self.get_layout().shape()
    }

    /// Draw the widget on the screen
    fn show(&mut self) -> Response {
        self.show_p(&mut *root_ui())
    }

    /// Draw the widget on the screen
    /// * `ui` is the Macroquad Ui engine
    fn show_p(&mut self, ui: &mut Ui) -> Response;
}

pub trait LayoutManager {
    /// Add the given widget to this widget's layout management
    /// * similar to `append` but consumes and returns self
    fn add(self, widget: impl Widget + 'static) -> Self;

    /// Add the given widget to this widget's layout management
    fn append(&mut self, widget: impl Widget + 'static);

    /// Get a reference to the widget by id
    fn get<T: AsRef<str>>(&self, id: T) -> Option<&Box<dyn Widget>>;
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
