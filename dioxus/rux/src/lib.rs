pub mod state;

pub const STYLES: &str = include_str!("./compiled_styles.css");

use once_cell::sync::Lazy;
use state::config::Config;
use std::path::PathBuf;

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    // Determine root path
    // 1. Development runs of examples will have a CWD of the `.../rux` root
    // TODO: 2. Development runs from another appliction?
    // TODO: 3. Production runs from another application?
    // TODO: 4. Production runs from WASM?
    let assets_path = PathBuf::from("assets");

    Config {
        // Themes path
        themes_path: assets_path.join("themes"),
    }
});

/// All essential symbols in a simple consumable way. Re-exports dioxus dependencies.
///
/// ### Examples
/// ```
/// use rux::prelude::*;
/// ```
pub mod prelude {
    // Re-exports
    pub use dioxus;
    pub use dioxus::core_macro::rsx;
    pub use dioxus::prelude::*;
    pub use dioxus_desktop;
    pub use dioxus_router;
    pub use dioxus_web;

    // Exports
    pub use crate::{
        state::{self, config},
        STYLES,
    };
}
