use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Response {
    /// Widget id that generated this response
    pub id: String,

    /// Mouse clicked this widget
    pub clicked: bool,

    /// Mouse is hovering over the widget
    pub hovered: bool,

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
