//! Button using Macroquad mimicking Android like UI behavior
//!
//! ### Example
//! ```
//! use button::prelude::*;
//! ```
mod button;

/// All essential symbols in a simple consumable form
///
/// ### Examples
/// ```
/// use button::prelude::*;
/// ```
pub mod prelude {
    // Re-exports
    pub use core::prelude::*;

    // Export internal types
    pub use crate::button::*;
}
