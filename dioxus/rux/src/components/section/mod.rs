use crate::elements::Label;
use dioxus::prelude::*;

#[derive(Props)]
pub struct SectionProps<'a> {
    section_label: String,
    section_description: String,
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn Section<'a>(cx: Scope<'a, SectionProps<'a>>) -> Element<'a> {
    cx.render(rsx!(
        div {
            class: "section",
            aria_label: "section",
            div {
                class: "section-info",
                aria_label: "section-info",
                Label {
                    text: cx.props.section_label.clone(),
                },
                p {
                    "{cx.props.section_description}"
                }
            },
            cx.props.children.is_some().then(|| rsx!(
                div {
                    class: "section-control",
                    aria_label: "section-control",
                    &cx.props.children
                }
            ))
        }
    ))
}
