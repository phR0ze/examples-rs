use common::icons::outline::Shape as Icon;
use common::icons::Icon as IconElement;
use dioxus::prelude::*;
use kit::elements::label::Label;

pub mod sidebar;
pub mod sub_pages;
#[derive(Props)]
pub struct SectionProps<'a> {
    section_label: String,
    section_description: String,
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn SettingSection<'a>(cx: Scope<'a, SectionProps<'a>>) -> Element<'a> {
    cx.render(rsx!(
        div {
            class: "settings-section",
            aria_label: "settings-section",
            div {
                class: "settings-info",
                aria_label: "settings-info",
                Label {
                    text: cx.props.section_label.clone(),
                },
                p {
                    "{cx.props.section_description}"
                }
            },
            cx.props.children.is_some().then(|| rsx!(
                div {
                    class: "settings-control",
                    aria_label: "settings-control",
                    &cx.props.children
                }
            ))
        }
    ))
}
