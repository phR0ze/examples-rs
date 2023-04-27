//! # RUX: Reusable UX for Dioxus
//!
//! `rux` aims to provide a simple set of elements, components and layouts for those that
//! don't want to start from scratch. It is based off a number of excellent projects
//! * Satellite-im's Uplink project
//! * dioxus-free-icons
//!
//! ### Icons
//! `rux` re-exports the dioxus-free-icons crate as `rux::icons` and `rux::elements::Icon`
//! to ergonomically blend the two crates together for ease of use.
//!
use include_more;
use once_cell::sync::Lazy;
use state::Theme;
use std::{
    fs,
    path::{Path, PathBuf},
};
use titlecase::titlecase;

#[cfg(any(windows, unix))]
use walkdir::WalkDir;

#[cfg(any(windows, unix))]
use state::Config;

// Public exports
// ****************************************************************************
pub mod components;
pub mod elements;
pub mod state;
pub use dioxus_free_icons::icons;

pub const STYLES: &str = include_str!("./compiled_styles.css");
include_more::include_files_as_strs! {
    static THEMES = {
        path: "assets/themes",
    };
}

#[cfg(any(windows, unix))]
pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    // Determine root path

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
    pub use dioxus::prelude::*;
    pub use dioxus_router;
    pub use include_more;
    pub use once_cell;
    pub use serde;
    pub use titlecase;

    #[cfg(any(windows, unix))]
    pub use dioxus_desktop;

    #[cfg(target_family = "wasm")]
    pub use dioxus_web;

    // Exports
    #[cfg(any(windows, unix))]
    pub use crate::{elements, state, STYLES};
}

/// Get the available system and user themes for a RUX application
///
/// ### Examples
/// ```no_run
/// use rux::state::config;
/// let themes = config::get_available_themes();
/// ```
pub fn get_available_themes() -> Vec<Theme> {
    let mut themes = vec![];

    // Add built in themes
    for file in crate::THEMES.iter() {
        let theme = Theme {
            filename: file.path.clone(),
            name: titlecase(&get_pretty_name(&file.path)).to_owned(),
            styles: file.data.clone(),
        };
        if !themes.contains(&theme) {
            themes.push(theme);
        }
    }

    // let mut add_to_themes = |themes_path| {
    //     for file in WalkDir::new(themes_path).into_iter().filter_map(|file| file.ok()) {
    //         if file.metadata().map(|x| x.is_file()).unwrap_or(false) {
    //             let theme_path = file.path().display().to_string();
    //             let pretty_theme_str = get_pretty_name(&theme_path);
    //             let pretty_theme_str = titlecase(&pretty_theme_str);

    //             let styles = fs::read_to_string(&theme_path).unwrap_or_default();

    //             let theme = Theme { filename: theme_path.to_owned(), name: pretty_theme_str.to_owned(), styles };
    //             if !themes.contains(&theme) {
    //                 themes.push(theme);
    //             }
    //         }
    //     }
    // };
    // add_to_themes(&crate::CONFIG.themes_path);
    // //add_to_themes(&crate::CONFIG.user_themes_path);

    themes.sort_by_key(|theme| theme.name.clone());
    themes.dedup();

    themes
}

// Simple theme name extraction from path
fn get_pretty_name<S: AsRef<str>>(name: S) -> String {
    let path = Path::new(name.as_ref());
    let last = path.file_name().and_then(|p| Path::new(p).file_stem()).unwrap_or_default();
    last.to_string_lossy().into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_pretty_name() {
        if cfg!(windows) {
            let r = get_pretty_name("c:\\pretty\\name2.scss");
            assert_eq!(r, String::from("name2"));
        } else {
            let r = get_pretty_name("pretty/name1.scss");
            assert_eq!(r, String::from("name1"));
        }
    }
}
