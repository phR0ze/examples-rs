use crate::prelude::*;

pub trait Widget {
    /// Get the widget's layout as a cloned reference
    fn layout_ref(&self) -> Layout;

    /// Draw the widget on the screen
    /// * `ui` is the Macroquad Ui engine
    fn show(&mut self, ui: &mut Ui);
}

pub trait LayoutManager {
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
