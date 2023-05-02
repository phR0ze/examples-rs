//! A simple container to center your content horizontally
//!
use dioxus::prelude::*;

#[derive(Props)]
pub struct ContainerProps<'a> {
    #[props(default)]
    is_widescreen: bool,

    #[props(default)]
    is_fullhd: bool,

    #[props(default)]
    is_max_desktop: bool,

    #[props(default)]
    is_max_widescreen: bool,

    #[props(default)]
    is_fluid: bool,

    children: Element<'a>,
}

/// The container is a simple utility layout that allows you to center content on larger viewports.
///
/// ### Properties
/// * `is_widescreen: bool`
/// * `is_fullhd: bool`
/// * `is_max_desktop: bool`
/// * `is_max_widescreen: bool`
/// * `is_fluid: bool`
#[allow(non_snake_case)]
pub fn Container<'a>(cx: Scope<'a, ContainerProps<'a>>) -> Element {
    let extra_class = if cx.props.is_widescreen {
        "is-widescreen"
    } else if cx.props.is_fullhd {
        "is-fullhd"
    } else if cx.props.is_max_desktop {
        "is-max-desktop"
    } else if cx.props.is_max_widescreen {
        "is-max-widescreen"
    } else if cx.props.is_fluid {
        "is-fluid"
    } else {
        ""
    };

    cx.render(rsx! {
        div {
            class: "container {extra_class}",
            &cx.props.children
        }
    })
}
