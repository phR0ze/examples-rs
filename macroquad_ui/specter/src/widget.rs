use crate::prelude::*;

pub trait Widget {
    /// Returns a reference clone to the Widget's layout
    fn layout_ref(&self) -> Layout;

    /// Draw the widget on the screen
    /// * `ui` is the Macroquad Ui engine
    fn show(&mut self, ui: &mut Ui);
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

    use super::*;

    #[test]
    fn test_widget() {
        //
    }
}
