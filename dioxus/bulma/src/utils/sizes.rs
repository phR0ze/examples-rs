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

//     /// Appends the converted CSS class to the given classes string
//     pub fn append_is_class(&self, classes: &str) -> String {
//         let size = self.to_string();
//         format!("{classes} is-{size}")
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

/// The `Button` and `Buttons` elements can accept 4 different sizes.
///
/// You can change the size of multiple buttons at once by using the `to_are_class`
/// or `append_are_class` with the `Buttons` function to change the size of multiple
/// buttons at once.
///
/// * Default: `normal`
#[derive(Clone, PartialEq)]
pub enum ButtonSizes {
    Small,
    Normal,
    Medium,
    Large,
}

impl ButtonSizes {
    /// Converts the enum to the singular `is` CSS class
    pub fn to_is_class(&self) -> String {
        format!("is-{}", self.to_string())
    }

    /// Converts the enum to the plural `are` CSS class
    pub fn to_are_class(&self) -> String {
        format!("are-{}", self.to_string())
    }

    /// Appends the converted `is` CSS class to the given classes string
    pub fn append_is_class(&self, classes: &str) -> String {
        let size = self.to_string();
        format!("{classes} is-{size}")
    }

    /// Appends the converted `are` CSS class to the given classes string
    pub fn append_are_class(&self, classes: &str) -> String {
        let size = self.to_string();
        format!("{classes} are-{size}")
    }
}

impl ToString for ButtonSizes {
    fn to_string(&self) -> String {
        match self {
            ButtonSizes::Small => "small",
            ButtonSizes::Normal => "normal",
            ButtonSizes::Medium => "medium",
            ButtonSizes::Large => "large",
        }
        .to_string()
    }
}

impl Default for ButtonSizes {
    fn default() -> Self {
        Self::Normal
    }
}