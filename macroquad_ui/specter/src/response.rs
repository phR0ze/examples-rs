use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Response {
    /// Widget id that generated this response
    pub id: String,

    /// Toggles the clicked result
    /// * this only works if your persisting the widget outside the game loop
    pub activated: bool,

    /// Mouse clicked this widget i.e. mouse down then back up
    pub clicked: bool,

    /// Mouse is hovering over the widget
    pub hovered: bool,

    /// Mouse is in a click down but not yet back up state
    pub mouse_down: bool,

    /// Responses gathered from child widgets
    pub responses: Vec<Response>,
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
