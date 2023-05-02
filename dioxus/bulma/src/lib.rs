//! Dioxus Bulma example
//!
mod utils;

// Exports
pub mod components;
pub mod elements;
pub mod layouts;

pub mod prelude {
    pub use crate::{get_bulma_css, utils::*};
    pub use dioxus::prelude::*;
}

// Get BULMA styles as a static str
pub fn get_bulma_css() -> &'static str {
    include_str!("./bulma.min.css")
}
