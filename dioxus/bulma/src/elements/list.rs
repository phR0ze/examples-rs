use dioxus::prelude::*;

#[allow(non_snake_case)]
#[derive(Props)]
pub struct ListProps<'a> {
    children: Element<'a>,
}

/// List
///
/// ### Properties
#[allow(non_snake_case)]
pub fn List<'a>(cx: Scope<'a, ListProps<'a>>) -> Element {
    let class = "list".to_string();

    cx.render(rsx! {
        ul {
            class: "{class}",
            &cx.props.children
        }
    })
}

#[allow(non_snake_case)]
#[derive(Props)]
pub struct ListItemProps<'a> {
    #[props(optional)]
    class: Option<String>,

    children: Element<'a>,
}

/// ListItem
///
/// ### Properties
/// * `class: Option<String>` is a space delimeted list of extra classes to apply
#[allow(non_snake_case)]
pub fn ListItem<'a>(cx: Scope<'a, ListItemProps<'a>>) -> Element {
    let mut class = "list-item".to_string();
    if let Some(extra) = &cx.props.class {
        class = format!("{class} {extra}")
    }

    cx.render(rsx! {
        ul {
            class: "{class}",
            &cx.props.children
        }
    })
}
