use derive_more::Display;

mod button;
mod label;
mod switch;
mod tooltip;
pub use button::Button;
pub use label::Label;
pub use switch::Switch;
pub use tooltip::{ArrowPosition, Tooltip};

/// Decides the look and feel of a button, also modifies some functionality.
#[derive(Clone, PartialEq, Eq, Copy, Display)]
pub enum Appearance {
    #[display(fmt = "default")]
    Default,

    #[display(fmt = "primary")]
    Primary,

    #[display(fmt = "secondary")]
    Secondary,

    #[display(fmt = "secondary-less")]
    SecondaryLess,

    #[display(fmt = "success")]
    Success,

    #[display(fmt = "danger")]
    Danger,

    #[display(fmt = "disabled")]
    Disabled,

    #[display(fmt = "transparent")]
    Transparent,
}
