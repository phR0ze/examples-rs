use crate::state::theme::Theme;
use std::{
    fs,
    path::{Path, PathBuf},
};
use titlecase::titlecase;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Config {
    /// Custom themes
    pub themes_path: PathBuf,
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

    let mut add_to_themes = |themes_path| {
        for file in WalkDir::new(themes_path).into_iter().filter_map(|file| file.ok()) {
            if file.metadata().map(|x| x.is_file()).unwrap_or(false) {
                let theme_path = file.path().display().to_string();
                let pretty_theme_str = get_pretty_name(&theme_path);
                let pretty_theme_str = titlecase(&pretty_theme_str);

                let styles = fs::read_to_string(&theme_path).unwrap_or_default();

                let theme = Theme { filename: theme_path.to_owned(), name: pretty_theme_str.to_owned(), styles };
                if !themes.contains(&theme) {
                    themes.push(theme);
                }
            }
        }
    };
    add_to_themes(&crate::CONFIG.themes_path);
    //add_to_themes(&crate::CONFIG.user_themes_path);

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
