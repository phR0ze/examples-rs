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

impl Colors {
    pub fn to_class(&self) -> String {
        format!("is-{}", self.to_string())
    }
    pub fn append_class(&self, class: &str) -> String {
        let color = self.to_string();
        format!("{class} is-{color}")
    }
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
