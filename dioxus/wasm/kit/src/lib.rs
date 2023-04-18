use std::path::PathBuf;

use anyhow::bail;

pub mod elements;
pub mod icons;

pub const STYLE: &str = include_str!("./compiled_styles.css");

/// Loads the script to string.
pub fn get_script(script: &'static str, uuid: &str) -> String {
    // The replace is needed because you can't have hyphens in javascript declarations.
    script.replace("DIUU", uuid).replace("SAFE_UUID", &uuid.replace('-', "_"))
}
