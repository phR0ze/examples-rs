//! Implements UI widgets using Macroquad to mimic Android like UI behaviors
//!
//! ### Example
//! ```
//! use mq_menu::prelude::*;
//! ```
//mod position;
//mod size;
//mod utils;

/// All essential symbols in a simple consumable form
///
/// ### Examples
/// ```
/// use mq_menu::prelude::*;
/// ```
pub mod prelude {
    // Re-exports
    pub use macroquad::{
        prelude::*,
        ui::{hash, root_ui, widgets, Id, Skin, Style, Ui},
    };

    // Export internal types
    //pub use crate::position::*;
    //pub use crate::size::*;
    //pub use crate::utils::*;
}
