use dioxus::{events::MouseEvent, prelude::*};

use crate::utils::*;

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
    size: Option<Sizes>,

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
    let mut classes = "button".to_string();

    if let Some(color) = &cx.props.color {
        let color_name = color.to_string();
        classes = format!("{classes} is-{color_name}");
    }

    if let Some(size) = &cx.props.size {
        let size_name = size.to_string();
        classes = format!("{classes} is-{size_name}");
    }

    if cx.props.is_light {
        classes += " is-light";
    }

    if cx.props.is_outlined {
        classes += " is-outlined";
    }

    if cx.props.is_inverted {
        classes += " is-inverted";
    }

    if cx.props.is_rounded {
        classes += " is-rounded";
    }

    if cx.props.is_fullwidth {
        classes += " is-fullwidth";
    }

    let state = &cx.props.state;
    let mut disabled = "false";
    if *state != ButtonState::Normal {
        match state {
            ButtonState::Normal => {},
            ButtonState::Hover => {
                classes += " is-hovered";
            },
            ButtonState::Focus => {
                classes += " is-focused";
            },
            ButtonState::Active => {
                classes += " is-active";
            },
            ButtonState::Loading => {
                classes += " is-loading";
            },
            ButtonState::Static => {
                classes += " is-static";
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
            class: "{classes}",
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
    size: Option<Sizes>,

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
    let mut classes = "buttons".to_string();

    if let Some(color) = &cx.props.color {
        let color_name = color.to_string();
        classes = format!("{classes} are-{color_name}");
    }

    if let Some(size) = &cx.props.size {
        let size_name = size.to_string();
        classes = format!("{classes} are-{size_name}");
    }

    if cx.props.are_light {
        classes += " are-light";
    }

    if cx.props.are_outlined {
        classes += " are-outlined";
    }

    if cx.props.are_inverted {
        classes += " are-inverted";
    }

    if cx.props.are_rounded {
        classes += " are-rounded";
    }

    if cx.props.are_fullwidth {
        classes += " are-fullwidth";
    }

    // TODO: finish are- implementation

    cx.render(rsx! {
        div {
            class: "{classes}",
            &cx.props.children
        }
    })
}
