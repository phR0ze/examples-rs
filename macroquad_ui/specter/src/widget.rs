use crate::prelude::*;

pub trait Widget {
    /// Adds the widget to this widgets layout management
    /// * `widget` is the widget being added
    fn append(&mut self, widget: impl Widget);

    /// Draw the widget on the screen
    /// * `ui` is the Macroquad Ui engine
    fn show(&mut self, ui: &mut Ui);
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
