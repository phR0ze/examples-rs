//! Widgeet
//!
use crate::prelude::*;

pub trait Widget {
    /// Returns a reference clone to the Widget's layout
    fn layout_ptr(&self) -> Layout;

    /// Get the widget's shape as a (position, size) tuple
    fn shape(&self) -> (Vec2, Vec2) {
        self.layout_ptr().shape()
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
