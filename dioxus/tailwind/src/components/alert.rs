use dioxus::prelude::*;

pub enum AlertState {
    Info,
}

impl ToString for AlertState {
    fn to_string(&self) -> String {
        match self {
            AlertState::Info => "text-blue-800 dark:text-blue-400",
        }
        .to_string()
    }
}

#[allow(non_snake_case)]
#[derive(Props)]
pub struct AlertProps<'a> {
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn Alert<'a>(cx: Scope<'a, AlertProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "p-4 mb-4 text-sm text-blue-800 rounded-lg bg-blue-50 dark:bg-gray-800 dark:text-blue-400",
            role: "alert",
            span {
                class: "font-semibold",
                "Info alert! "
            }
            &cx.props.children
        }
    })
}

#[allow(non_snake_case)]
pub fn AlertInfo<'a>(cx: Scope<'a, AlertProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "p-4 mb-4 text-sm text-blue-800 rounded-lg bg-blue-50 dark:bg-gray-800 dark:text-blue-400",
            role: "alert",
            span {
                class: "font-semibold",
                "Info alert! "
            }
            &cx.props.children
        }
    })
}
