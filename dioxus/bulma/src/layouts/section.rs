//! A simple container to divide your page into sections
//!
use dioxus::prelude::*;

#[derive(Props)]
pub struct SectionProps<'a> {
    #[props(default)]
    class: Option<&'a str>,

    children: Element<'a>,
}

/// Small section container to divide your page
///
/// ### Properties
#[allow(non_snake_case)]
pub fn Section<'a>(cx: Scope<'a, SectionProps<'a>>) -> Element {
    let mut class = "section".to_string();
    if let Some(extra) = cx.props.class {
        class = format!("{class} {extra}");
    }

    cx.render(rsx! {
        div {
            class: "{class}",
            &cx.props.children
        }
    })
}

/// Medium section container to divide your page
///
/// ### Properties
#[allow(non_snake_case)]
pub fn SectionMedium<'a>(cx: Scope<'a, SectionProps<'a>>) -> Element {
    let mut class = "section is-medium".to_string();
    if let Some(extra) = cx.props.class {
        class = format!("{class} {extra}");
    }

    cx.render(rsx! {
        div {
            class: "{class}",
            &cx.props.children
        }
    })
}
