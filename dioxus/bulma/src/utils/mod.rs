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

/// Responsive image ratios
/// * Bulma docs https://bulma.io/documentation/elements/image/#responsive-images-with-ratios
#[derive(Clone, PartialEq)]
pub enum Ratios {
    Square,
    OneByOne,
    FiveByFour,
    FourByThree,
    ThreeByTwo,
    FiveByThree,
    SixteenByNine,
    TwoByOne,
    ThreeByOne,
    FourByFive,
    ThreeByFour,
    TwoByThree,
    ThreeByFive,
    NineBySixteen,
    OneByTwo,
    OneByThree,
}

impl fmt::Display for Ratios {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Ratios::Square => "square",
                Ratios::OneByOne => "1by1",
                Ratios::FiveByFour => "5by4",
                Ratios::FourByThree => "4by3",
                Ratios::ThreeByTwo => "3by2",
                Ratios::FiveByThree => "5by3",
                Ratios::SixteenByNine => "16by9",
                Ratios::TwoByOne => "2by1",
                Ratios::ThreeByOne => "3by1",
                Ratios::FourByFive => "4by5",
                Ratios::ThreeByFour => "3by4",
                Ratios::TwoByThree => "2by3",
                Ratios::ThreeByFive => "3by5",
                Ratios::NineBySixteen => "9by16",
                Ratios::OneByTwo => "1by2",
                Ratios::OneByThree => "1by3",
            }
        )
    }
}
