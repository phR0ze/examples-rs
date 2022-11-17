//! Immediate mode Ui toolkit
//!
//! ### Example
//! ```
//! use specter::prelude::*;
//! ```
#[macro_use]
mod macros;

mod align;
// mod button;
mod fps;
mod frame;
mod image;
mod label;
mod layout;
mod panel;
mod response;
mod utils;
mod widget;

/// All essential symbols in a simple consumable form
///
/// ### Examples
/// ```
/// use specter::prelude::*;
/// ```
pub mod prelude {
    // Re-exports
    pub use macroquad::{
        color::*,
        math::*,
        shapes::*,
        texture::Texture2D,
        ui::{hash, root_ui, widgets, Drag, Id, Skin, Style, Ui},
        window::*,
    };

    // Export macros by name
    pub use crate::id;

    // Export internal types
    pub use crate::align::*;
    // pub use crate::button::*;
    pub use crate::fps::*;
    pub use crate::frame::*;
    pub use crate::image::*;
    pub use crate::label::*;
    pub use crate::layout::*;
    pub use crate::macros::*;
    pub use crate::panel::*;
    pub use crate::response::*;
    pub use crate::utils::*;
    pub use crate::widget::*;
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
