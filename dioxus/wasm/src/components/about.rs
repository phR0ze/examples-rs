#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn About(cx: Scope) -> Element {
    cx.render(rsx!(p {
        b {"Dioxus example"}
        " Experimenting with Dioxus to understand its capabilities."
    }))
}
