use common::icons::outline::Shape as Icon;
use dioxus::prelude::*;
use kit::{
    components::section::Section,
    elements::{
        button::Button,
        checkbox::Checkbox,
        switch::Switch,
        tooltip::{ArrowPosition, Tooltip},
        Appearance,
    },
    STYLE,
};
use ui::{utils::get_available_themes, APP_STYLE};

fn main() {
    dioxus_desktop::launch_cfg(
        App,
        dioxus_desktop::Config::new().with_window(
            dioxus_desktop::WindowBuilder::new()
                .with_title("crux")
                .with_resizable(true)
                // Provides rounded window corner effect
                .with_transparent(true)
                // Turns off standard window manager controls
                .with_decorations(false)
                // We start the min inner size smaller because the prelude pages like unlock can be rendered much smaller.
                .with_min_inner_size(dioxus_desktop::LogicalSize::new(300.0, 350.0))
                .with_inner_size(dioxus_desktop::LogicalSize::new(950.0, 600.0)),
        ),
    )
}

#[allow(non_snake_case)]
fn TitleBar(cx: Scope) -> Element {
    let desktop = dioxus_desktop::use_window(cx);
    cx.render(rsx!(
        div {
            id: "titlebar",
            onmousedown: move |_| { desktop.drag(); },
            div {
                class: "controls",
                Button {
                    aria_label: "minimize-button".into(),
                    icon: Icon::Minus,
                    appearance: Appearance::Transparent,
                    onpress: move |_| {
                        desktop.set_minimized(true);
                    }
                },
                Button {
                    aria_label: "square-button".into(),
                    icon: Icon::Square2Stack,
                    appearance: Appearance::Transparent,
                    onpress: move |_| {
                        desktop.set_maximized(!desktop.is_maximized());
                    }
                },
                Button {
                    aria_label: "close-button".into(),
                    icon: Icon::XMark,
                    appearance: Appearance::Transparent,
                    onpress: move |_| {
                        desktop.close();
                    }
                },
            },
        }
    ))
}

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    let theme = get_available_themes().iter().find(|x| x.name == "Nord").unwrap().styles.clone();

    cx.render(rsx! {
        style { "{STYLE} {APP_STYLE} {theme}" },
        div {
            TitleBar{},
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
