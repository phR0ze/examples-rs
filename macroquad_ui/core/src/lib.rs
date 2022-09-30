//! Core utilities and functions for UI development
//!
//! ### Example
//! ```
//! use core::prelude::*;
//! ```
mod position;
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
        ui::{hash, root_ui, widgets, Id, Skin, Style, Ui},
    };

    // Export internal types
    pub use crate::position::*;
    pub use crate::size::*;
    pub use crate::utils::*;
}
