//! Dioxus Bulma example
//!
mod utils;

// Exports
pub mod components;
pub mod elements;
pub mod layouts;
pub mod icons {
    pub use dioxus_free_icons::icons::*;
    pub use dioxus_free_icons::{Icon, IconProps, IconShape};
}
pub use dioxus;
pub use dioxus_router;
pub use fermi;

#[cfg(any(windows, unix))]
pub use dioxus_desktop;

#[cfg(target_family = "wasm")]
pub use dioxus_web;

pub mod prelude {
    pub use crate::{get_bulma_css, utils::*};
    pub use dioxus;
    pub use dioxus::prelude::*;
    pub use dioxus_router::{self, use_route, use_router, Route, Router};
    pub use fermi::{self, use_atom_ref, use_init_atom_root, AtomRef};

    #[cfg(any(windows, unix))]
    pub use dioxus_desktop;

    #[cfg(target_family = "wasm")]
    pub use dioxus_web;
}

// Get BULMA styles as a static str
pub fn get_bulma_css() -> &'static str {
    include_str!("./bulma.min.css")
}
