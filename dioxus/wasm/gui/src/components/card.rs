#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props)]
pub struct CardProps<'a> {
    title: &'a str,
}

pub fn Card<'a>(cx: Scope<'a, CardProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            img {
                class: "h-9",
                width: "auto",
                alt: "",
                src: "https://shuffle.dev/yofte-assets/logos/yofte-logo.svg",
            },
            h1 { "{cx.props.title}" }
            h3 { "{cx.props.title}" }
        }
    })
}
