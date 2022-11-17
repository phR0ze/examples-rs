use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Response {
    /// Mouse clicked this widget
    pub clicked: bool,

    /// Mouse is hovering over the widget
    pub hovered: bool,
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
