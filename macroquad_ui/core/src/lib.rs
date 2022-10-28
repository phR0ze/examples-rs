//! Core utilities and functions for UI development
//!
//! ### Example
//! ```
//! use core::prelude::*;
//! ```
#[macro_use]
mod macros;

mod align;
mod button;
mod button1;
mod fps;
mod frame;
mod label;
mod layout;
mod panel;
mod size;
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
    pub use crate::button::*;
    pub use crate::button1::*;
    pub use crate::fps::*;
    pub use crate::frame::*;
    pub use crate::label::*;
    pub use crate::layout::*;
    pub use crate::macros::*;
    pub use crate::panel::*;
    pub use crate::size::*;
    pub use crate::utils::*;
}
