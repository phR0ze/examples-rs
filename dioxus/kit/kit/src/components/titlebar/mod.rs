use crate::elements::{button::Button, Appearance};
use common::icons::outline::Shape as Icon;
use common::icons::Icon as IconElement;
use dioxus::prelude::*;

#[allow(dead_code)]
#[derive(PartialEq, Props)]
pub struct Props {
    #[props(optional)]
    icon: Option<Icon>,
    #[props(optional)]
    text: Option<String>,
    #[props(optional)]
    link: Option<String>,
}

// Custom window titlebar with custom window controls
#[allow(dead_code)]
#[allow(non_snake_case)]
pub fn Titlebar<'a>(cx: Scope<'a, Props>) -> Element<'a> {
    let desktop = dioxus_desktop::use_window(cx);
    let text = cx.props.text.clone().unwrap_or_default();
    cx.render(rsx!(
        div {
            id: "titlebar",
            aria_label: "Title bar",
            onmousedown: move |_| { desktop.drag(); },
            div {
                id: "titlebar-message",
                aria_label: "titlebar-message",
                IconElement {
                    icon: cx.props.icon.unwrap_or(Icon::Beaker)
                },
                p {
                    if let Some(link) = cx.props.link.clone() {
                        rsx! {
                            div {
                                onclick: move |_| {
                                    let _ = open::that(&link);
                                },
                                "{text}"
                            }
                        }
                    } else {
                        rsx! {
                            div {
                                "{text}"
                            }
                        }
                    }
                },
            },
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
