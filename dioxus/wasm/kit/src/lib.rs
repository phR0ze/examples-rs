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

/// Determines the location of the assets
fn get_assets_dir() -> anyhow::Result<PathBuf> {
    let assets_path = if cfg!(target_os = "windows") {
        PathBuf::from(r"..\extra")
    } else if cfg!(target_os = "linux") {
        PathBuf::from("dist/assets/images")
    } else if cfg!(target_os = "macos") {
        let exe_path = std::env::current_exe()?;
        exe_path
            .parent()
            .and_then(|x| x.parent())
            .map(|x| x.join("Resources").join("extra"))
            .ok_or(anyhow::format_err!("failed to get MacOs resources dir"))?
    } else {
        bail!("unknown OS type. failed to copy assets");
    };

    Ok(assets_path)
}
