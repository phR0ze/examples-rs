use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Colors {
    White,
    Light,
    Dark,
    Black,
    Text,
    Ghost,
    Primary,
    Link,
    Info,
    Success,
    Warning,
    Danger,
}

impl ToString for Colors {
    fn to_string(&self) -> String {
        match self {
            Colors::White => "white",
            Colors::Light => "light",
            Colors::Dark => "dark",
            Colors::Black => "black",
            Colors::Text => "text",
            Colors::Ghost => "ghost",
            Colors::Primary => "primary",
            Colors::Link => "link",
            Colors::Info => "info",
            Colors::Success => "success",
            Colors::Warning => "warning",
            Colors::Danger => "danger",
        }
        .to_string()
    }
}

#[derive(Clone, PartialEq)]
pub enum Sizes {
    Small,
    Normal,
    Medium,
    Large,
}

impl ToString for Sizes {
    fn to_string(&self) -> String {
        match self {
            Sizes::Small => "small",
            Sizes::Normal => "normal",
            Sizes::Medium => "medium",
            Sizes::Large => "large",
        }
        .to_string()
    }
}

impl Default for Sizes {
    fn default() -> Self {
        Self::Normal
    }
}
