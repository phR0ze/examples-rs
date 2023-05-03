//! A simple container to divide your page into sections
//!
use dioxus::prelude::*;

#[derive(Props)]
pub struct SectionProps<'a> {
    children: Element<'a>,
}

/// A simple container to divide your page into sections
///
/// ### Properties
#[allow(non_snake_case)]
pub fn Section<'a>(cx: Scope<'a, SectionProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "section",
            &cx.props.children
        }
    })
}
