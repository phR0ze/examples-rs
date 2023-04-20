#![allow(non_snake_case)]

use dioxus::prelude::*;
use kit::{components::section::Section, elements::switch::Switch, STYLE};

fn main() {
    dioxus_desktop::launch_cfg(App, dioxus_desktop::Config::new());
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        style { "{STYLE}" },
        div {
            Section {
                section_label: "Inactive switch".into(),
                section_description: "Example of a switch that is inactive by default".into(),
                Switch {
                    active: false,
                },
            },
            Section {
                section_label: "Active switch".into(),
                section_description: "Example of a switch that is active by default".into(),
                Switch {
                    active: true,
                }
            }
        }
    })
}
