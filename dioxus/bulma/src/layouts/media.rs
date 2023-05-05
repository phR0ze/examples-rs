//! Media is a UI layout object perfect for repeatable and nestable content
//!
use dioxus::prelude::*;

#[derive(Props)]
pub struct MediaProps<'a> {
    children: Element<'a>,
}

/// Media is a UI layout object perfect for repeatable and nestable content
///
/// ### Properties
/// * `children: Element<'a>` any other child objects to be contained inside this object
#[allow(non_snake_case)]
pub fn Media<'a>(cx: Scope<'a, MediaProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "media",
            &cx.props.children
        }
    })
}

#[derive(Props)]
pub struct MediaLeftProps<'a> {
    children: Element<'a>,
}

/// MediaLeft is a UI layout object perfect for repeatable and nestable content
///
/// ### Properties
/// * `children: Element<'a>` any other child objects to be contained inside this object
#[allow(non_snake_case)]
pub fn MediaLeft<'a>(cx: Scope<'a, MediaLeftProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "media-left",
            &cx.props.children
        }
    })
}

#[derive(Props)]
pub struct MediaContentProps<'a> {
    children: Element<'a>,
}

/// MediaContent is a UI layout object perfect for repeatable and nestable content
///
/// ### Properties
/// * `children: Element<'a>` any other child objects to be contained inside this object
#[allow(non_snake_case)]
pub fn MediaContent<'a>(cx: Scope<'a, MediaContentProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "media-content",
            &cx.props.children
        }
    })
}
