#![allow(non_snake_case)]

use common::icons::outline::Shape as Icon;
use dioxus::prelude::*;
use kit::{
    elements::{
        button::Button,
        tooltip::{ArrowPosition, Tooltip},
    },
    STYLE,
};

fn main() {
    dioxus_desktop::launch_cfg(App, dioxus_desktop::Config::new());
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        style { "{STYLE}" },
        div {
            Button {
                text: "Default".into(),
                appearance: kit::elements::Appearance::Default,
                icon: Icon::Check,
            },
            Button {
                text: "Primary".into(),
                appearance: kit::elements::Appearance::Primary,
                icon: Icon::Check,
            },
            Button {
                text: "Secondary".into(),
                appearance: kit::elements::Appearance::Secondary,
                icon: Icon::Check,
            },
            Button {
                text: "SecondaryLess".into(),
                appearance: kit::elements::Appearance::SecondaryLess,
                icon: Icon::Check,
            },
            Button {
                text: "Success".into(),
                appearance: kit::elements::Appearance::Success,
                icon: Icon::Check,
            },
            Button {
                text: "Danger".into(),
                appearance: kit::elements::Appearance::Danger,
                icon: Icon::Check,
            },
            Button {
                text: "Disabled".into(),
                appearance: kit::elements::Appearance::Disabled,
                icon: Icon::Check,
            },
            Button {
                text: "Transparent".into(),
                appearance: kit::elements::Appearance::Transparent,
                icon: Icon::Check,
            }
            Button {
                text: "With Badge".into(),
                with_badge: "Badge".into(),
                appearance: kit::elements::Appearance::Default,
                icon: Icon::Check,
            },

            // Small button only provides room for the Icon and no Text
            p {
                "small: true"
            },
            Button {
                small: true,
                icon: Icon::Check,
            },
            p {}, p {}

            // Custom tooltip
            Button {
                text: "Custom Tooltip".into(),
                icon: Icon::Cog6Tooth,
                appearance: kit::elements::Appearance::Primary,
                tooltip: cx.render(rsx!(
                    Tooltip {
                        arrow_position: ArrowPosition::Bottom,
                        text: String::from("Settings")
                    }
                )),
            }
        }
    })
}
