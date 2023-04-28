use crate::{
    elements::{Appearance, Button},
    icons::{HiOutlineIcon, Icon},
};
use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct Props {
    #[props(optional)]
    icon: Option<HiOutlineIcon>,
    #[props(optional)]
    text: Option<String>,
    #[props(optional)]
    link: Option<String>,
}

// Custom window titlebar with custom window controls
#[allow(non_snake_case)]
#[cfg(any(windows, unix))]
pub fn TitleBar<'a>(cx: Scope<'a, Props>) -> Element<'a> {
    let desktop = dioxus_desktop::use_window(cx);
    let text = cx.props.text.clone().unwrap_or_default();
    cx.render(rsx!(
        div {
            id: "titlebar",
            aria_label: "Window Ctl",
            onmousedown: move |_| { desktop.drag(); },
            div {
                id: "titlebar-message",
                aria_label: "titlebar-message",
                Icon {
                    icon: cx.props.icon.unwrap_or(HiOutlineIcon::Beaker)
                },
                p {
                    if let Some(link) = cx.props.link.clone() {
                        rsx! {
                            div {
                                // onclick: move |_| {
                                //     let _ = open::that(&link);
                                // },
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
                    icon: HiOutlineIcon::Minus,
                    appearance: Appearance::Transparent,
                    onpress: move |_| {
                        desktop.set_minimized(true);
                    }
                },
                Button {
                    aria_label: "square-button".into(),
                    icon: HiOutlineIcon::Square2Stack,
                    appearance: Appearance::Transparent,
                    onpress: move |_| {
                        desktop.set_maximized(!desktop.is_maximized());
                    }
                },
                Button {
                    aria_label: "close-button".into(),
                    icon: HiOutlineIcon::XMark,
                    appearance: Appearance::Transparent,
                    onpress: move |_| {
                        desktop.close();
                    }
                },
            },
        }
    ))
}
