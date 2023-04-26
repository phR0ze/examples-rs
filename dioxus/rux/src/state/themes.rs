use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};
use titlecase::titlecase;

#[cfg(any(windows, unix))]
use walkdir::WalkDir;

#[derive(Eq, Clone, Debug, Default, Deserialize, Serialize)]
pub struct Theme {
    pub filename: String,
    pub name: String,
    pub styles: String,
}

impl PartialEq for Theme {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}
