use common::icons;
use dioxus::prelude::*;
use warp::logging::tracing::log;

use crate::elements::{button::Button, Appearance};

#[derive(Props)]
pub struct Props<'a> {
    #[props(optional)]
    title: Option<String>,
    #[props(optional)]
    with_back_button: Option<bool>,
    #[props(optional)]
    onback: Option<EventHandler<'a>>,
    #[props(optional)]
    controls: Option<Element<'a>>,
    #[props(optional)]
    children: Option<Element<'a>>,
}

/// If enabled, it will render the bool
pub fn show_back_button(cx: &Scope<Props>) -> bool {
    cx.props.with_back_button.unwrap_or(false)
}

/// Emit the back button event
pub fn emit(cx: &Scope<Props>) {
    match &cx.props.onback {
        Some(f) => f.call(()),
        None => {},
    }
}

#[allow(non_snake_case)]
pub fn Topbar<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    log::trace!("rendering topbar");
    let title = cx.props.title.clone();
    cx.render(rsx!(
        div {
            class: "topbar",
            aria_label: "Topbar",
            (show_back_button(&cx)).then(|| rsx!(
                Button {
                    aria_label: "back-button".into(),
                    icon: icons::outline::Shape::ChevronLeft,
                    onpress: move |_| emit(&cx),
                    appearance: Appearance::Secondary
                }
            )),
            if title.is_some() {
                rsx! {
                    div {
                        class: "topbar-title",
                        "{title.unwrap()}"
                    }
                }
            }
            div {
                class: "topbar-children",
                cx.props.children.as_ref()
            },
            div {
                class: "topbar-controls",
                cx.props.controls.as_ref()
            }
        }
    ))
}
