use crate::utils::*;
use dioxus::{events::MouseEvent, prelude::*};

#[derive(PartialEq)]
pub enum ButtonState {
    Normal,
    Hover,
    Focus,
    Active,
    Loading,
    Static,
    Disabled,
}

impl Default for ButtonState {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Props)]
pub struct ButtonProps<'a> {
    #[props(optional)]
    r#type: Option<&'a str>,

    #[props(optional)]
    color: Option<Colors>,

    #[props(optional)]
    size: Option<ButtonSizes>,

    #[props(default)]
    state: ButtonState,

    #[props(default)]
    is_light: bool,

    #[props(default)]
    is_outlined: bool,

    #[props(default)]
    is_inverted: bool,

    #[props(default)]
    is_rounded: bool,

    #[props(default)]
    is_fullwidth: bool,

    #[props(default)]
    onclick: EventHandler<'a, MouseEvent>,

    #[props(default)]
    onmousedown: EventHandler<'a, MouseEvent>,

    #[props(default)]
    onmouseup: EventHandler<'a, MouseEvent>,

    children: Element<'a>,
}

/// Button
///
/// ### Properties
/// * ``
pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {
    let mut class = "button".to_string();

    if let Some(color) = &cx.props.color {
        class = color.append_class(&class);
    }

    if let Some(size) = &cx.props.size {
        class = size.append_is_class(&class);
    }

    if cx.props.is_light {
        class += " is-light";
    }

    if cx.props.is_outlined {
        class += " is-outlined";
    }

    if cx.props.is_inverted {
        class += " is-inverted";
    }

    if cx.props.is_rounded {
        class += " is-rounded";
    }

    if cx.props.is_fullwidth {
        class += " is-fullwidth";
    }

    let state = &cx.props.state;
    let mut disabled = "false";
    if *state != ButtonState::Normal {
        match state {
            ButtonState::Normal => {},
            ButtonState::Hover => {
                class += " is-hovered";
            },
            ButtonState::Focus => {
                class += " is-focused";
            },
            ButtonState::Active => {
                class += " is-active";
            },
            ButtonState::Loading => {
                class += " is-loading";
            },
            ButtonState::Static => {
                class += " is-static";
            },
            ButtonState::Disabled => {
                disabled = "true";
            },
        }
    }

    let mut button_type = "button";
    if let Some(t) = cx.props.r#type {
        button_type = t;
    }

    cx.render(rsx! {
        button {
            class: "{class}",
            r#type: "{button_type}",
            disabled: "{disabled}",
            onclick: move |evt| cx.props.onclick.call(evt),
            onmousedown: move |evt| cx.props.onmousedown.call(evt),
            onmouseup: move |evt| cx.props.onmouseup.call(evt),
            &cx.props.children
        }
    })
}

#[derive(Props)]
pub struct ButtonsProps<'a> {
    #[props(optional)]
    r#type: Option<&'a str>,

    #[props(optional)]
    color: Option<Colors>,

    #[props(optional)]
    size: Option<ButtonSizes>,

    #[props(default)]
    state: ButtonState,

    #[props(default)]
    are_light: bool,

    #[props(default)]
    are_outlined: bool,

    #[props(default)]
    are_inverted: bool,

    #[props(default)]
    are_rounded: bool,

    #[props(default)]
    are_fullwidth: bool,

    #[props(default)]
    onclick: EventHandler<'a, MouseEvent>,

    #[props(default)]
    onmousedown: EventHandler<'a, MouseEvent>,

    #[props(default)]
    onmouseup: EventHandler<'a, MouseEvent>,

    children: Element<'a>,
}

/// Buttons
pub fn Buttons<'a>(cx: Scope<'a, ButtonsProps<'a>>) -> Element {
    let mut class = "buttons".to_string();

    if let Some(color) = &cx.props.color {
        let color_name = color.to_string();
        class = format!("{class} are-{color_name}");
    }

    if let Some(size) = &cx.props.size {
        let size_name = size.to_string();
        class = format!("{class} are-{size_name}");
    }

    if cx.props.are_light {
        class += " are-light";
    }

    if cx.props.are_outlined {
        class += " are-outlined";
    }

    if cx.props.are_inverted {
        class += " are-inverted";
    }

    if cx.props.are_rounded {
        class += " are-rounded";
    }

    if cx.props.are_fullwidth {
        class += " are-fullwidth";
    }

    // TODO: finish are- implementation

    cx.render(rsx! {
        div {
            class: "{class}",
            &cx.props.children
        }
    })
}
