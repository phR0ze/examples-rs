//! Menu combines Macroquad components to mimic Android like UI behavior
//!
//! ### Example
//! ```
//! use menu::prelude::*;
//! ```
mod menu;

/// All essential symbols in a simple consumable form
///
/// ### Examples
/// ```
/// use menu::prelude::*;
/// ```
pub mod prelude {
    // Re-exports
    pub use button::prelude::*;
    pub use core::prelude::*;
    pub use group::prelude::*;

    // Export internal types
    pub use crate::menu::*;
}
