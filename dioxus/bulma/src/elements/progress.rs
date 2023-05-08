use dioxus::prelude::*;

use crate::utils::*;

#[allow(non_snake_case)]
#[derive(Props, PartialEq)]
pub struct ProgressProps {
    #[props(default)]
    max: u16,

    #[props(default)]
    value: u16,

    #[props(optional)]
    size: Option<Sizes>,

    #[props(optional)]
    color: Option<Colors>,
}

#[allow(non_snake_case)]
pub fn Progress(cx: Scope<ProgressProps>) -> Element {
    let mut class = "progress".to_string();

    if cx.props.size.is_some() {
        class += &format!(" is-{}", cx.props.size.as_ref().unwrap().to_string());
    }

    if cx.props.color.is_some() {
        class += &format!(" is-{}", cx.props.color.as_ref().unwrap().to_string());
    }

    if cx.props.value != 0 {
        cx.render(rsx! {
            progress {
                class: "{class}",
                value: "{cx.props.value}",
                max: "{cx.props.max}",
            }
        })
    } else {
        cx.render(rsx! {
            progress {
                class: "{class}",
                max: "{cx.props.max}",
            }
        })
    }
}
