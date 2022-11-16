use crate::prelude::*;

pub trait Widget {
    /// Returns a reference clone to the Widget's layout
    fn layout_ref(&self) -> Layout;

    /// Draw the widget on the screen
    /// * `ui` is the Macroquad Ui engine
    fn show(&mut self, ui: &mut Ui);
}

pub trait LayoutManager {
    /// Adds the widget to this widget's layout management
    /// * alias for `append` that consumes the caller
    /// * `widget` is the widget being added
    fn add(self, widget: impl Widget + 'static) -> Self;

    /// Adds the widget to this widget's layout management
    /// * `widget` is the widget being added
    fn append(&mut self, widget: impl Widget + 'static);
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_widget() {
        //
    }
}
