//! Core utilities and functions for UI development
//!
//! ### Example
//! ```
//! use core::prelude::*;
//! ```
#[macro_use]
mod macros;

mod button;
mod fps;
mod frame;
mod label;
mod position;
mod size;
mod style;
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
    pub use crate::gid;

    // Export internal types
    pub use crate::button::*;
    pub use crate::fps::*;
    pub use crate::frame::*;
    pub use crate::label::*;
    pub use crate::macros::*;
    pub use crate::position::*;
    pub use crate::size::*;
    pub use crate::style::*;
    pub use crate::utils::*;
}
