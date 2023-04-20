#![allow(non_snake_case)]

use common::icons::outline::Shape as Icon;
use dioxus::prelude::*;
use kit::{
    components::section::Section,
    elements::{
        button::Button,
        checkbox::Checkbox,
        switch::Switch,
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
            Section {
                section_label: "Checkbox - unchecked".into(),
                section_description: "Example of a checkbox that is unchecked".into(),
                Checkbox{
                    disabled: false,
                    width: "1.5em".into(),
                    height: "1.5em".into(),
                    is_checked: false,
                    on_click: move |_| { },
                },
            },
            Section {
                section_label: "Checkbox - checked".into(),
                section_description: "Example of a checkbox that is checked".into(),
                Checkbox{
                    disabled: false,
                    width: "1.5em".into(),
                    height: "1.5em".into(),
                    is_checked: true,
                    on_click: move |_| { },
                },
            },
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
            },
            div {
                style: "border-bottom: 1px solid var(--border-color)",
                p {
                    "Button Examples",
                },
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
            },
            Section {
                section_label: "Small Button".into(),
                section_description: "Setting small=true provides room only for the icon".into(),
                Button {
                    small: true,
                    icon: Icon::Check,
                },
            },
            Section {
                section_label: "Custom Tooltip".into(),
                section_description: "Creating a custom tool tip for the button".into(),
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
                },
            }
        }
    })
}
