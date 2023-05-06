//! Sizing for various elements like Buttons and text
//!

// /// Typography can make use of 7 sizes
// #[derive(Clone, PartialEq)]
// pub enum TextSizes {
//     Size1,
//     Size2,
//     Size3,
//     Size4,
//     Size5,
//     Size6,
//     Size7,
// }

// impl TextSizes {
//     /// Converts the Sizes enum to a CSS class
//     pub fn to_class(&self) -> String {
//         format!("is-size-{}", self.to_string())
//     }

//     /// Appends the converted CSS class to the given class string
//     pub fn append_is_class(&self, class: &str) -> String {
//         let size = self.to_string();
//         format!("{class} is-{size}")
//     }
// }

// impl ToString for Sizes {
//     fn to_string(&self) -> String {
//         match self {
//             Sizes::Small => "small",
//             Sizes::Normal => "normal",
//             Sizes::Medium => "medium",
//             Sizes::Large => "large",
//         }
//         .to_string()
//     }
// }

// impl Default for Sizes {
//     fn default() -> Self {
//         Self::Normal
//     }
// }

/// There are 4 general size values that many non text elements will accept including
/// Buttons and Progress bars.
///
/// You can change the size of multiple buttons at once by using the `to_are_class`
/// or `append_are_class` with the `Buttons` function to change the size of multiple
/// buttons at once.
///
/// * Default: `normal`
#[derive(Clone, PartialEq)]
pub enum Sizes {
    Small,
    Normal,
    Medium,
    Large,
}

impl Sizes {
    /// Converts the enum to the singular `is` CSS class
    pub fn to_is_class(&self) -> String {
        format!("is-{}", self.to_string())
    }

    /// Converts the enum to the plural `are` CSS class
    pub fn to_are_class(&self) -> String {
        format!("are-{}", self.to_string())
    }

    /// Appends the converted `is` CSS class to the given class string
    pub fn append_is_class(&self, class: &str) -> String {
        let size = self.to_string();
        format!("{class} is-{size}")
    }

    /// Appends the converted `are` CSS class to the given class string
    pub fn append_are_class(&self, class: &str) -> String {
        let size = self.to_string();
        format!("{class} are-{size}")
    }
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
