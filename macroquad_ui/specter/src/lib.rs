//! Immediate mode Ui toolkit
//!
//! ### Example
//! ```
//! use core::prelude::*;
//! ```
#[macro_use]
mod macros;

mod align;
// mod button;
mod fps;
mod frame;
mod label;
mod layout;
mod panel;
mod utils;

/// All essential symbols in a simple consumable form
///
/// ### Examples
/// ```
/// use core::prelude::*;
/// ```
pub mod prelude {
    // Re-exports
    pub use macroquad::{
        color::colors,
        prelude::*,
        ui::{hash, root_ui, widgets, Drag, Id, Skin, Style, Ui},
    };

    // Export macros by name
    pub use crate::id;

    // Export internal types
    pub use crate::align::*;
    // pub use crate::button::*;
    pub use crate::fps::*;
    pub use crate::frame::*;
    pub use crate::label::*;
    pub use crate::layout::*;
    // pub use crate::macros::*;
    pub use crate::panel::*;
    pub use crate::utils::*;
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
