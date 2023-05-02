use dioxus::prelude::*;

#[allow(non_snake_case)]
#[derive(Props)]
pub struct TitleProps<'a> {
    #[props(optional)]
    size: Option<u8>,

    #[props(default)]
    is_spaced: bool,

    children: Element<'a>,
}

/// Title
///
/// ### Properties
/// `is_spaced: bool` allows you to maintain the normal spacing between titles and subtitles
#[allow(non_snake_case)]
pub fn Title<'a>(cx: Scope<'a, TitleProps<'a>>) -> Element {
    let mut classes = "title".to_string();

    if let Some(size) = cx.props.size {
        classes = format!("{classes} is-{size}");
    }

    if cx.props.is_spaced {
        classes += " is-spaced";
    }

    cx.render(rsx! {
        p {
            class: "{classes}",
            &cx.props.children
        }
    })
}

#[allow(non_snake_case)]
#[derive(Props)]
pub struct SubTitleProps<'a> {
    #[props(optional)]
    size: Option<u8>,

    #[props(default)]
    is_spaced: bool,

    children: Element<'a>,
}

/// SubTitle
///
/// ### Properties
/// `is_spaced: bool` allows you to maintain the normal spacing between titles and subtitles
#[allow(non_snake_case)]
pub fn SubTitle<'a>(cx: Scope<'a, TitleProps<'a>>) -> Element {
    let mut classes = "subtitle".to_string();

    if let Some(size) = cx.props.size {
        classes = format!("{classes} is-{size}");
    }

    if cx.props.is_spaced {
        classes += " is-spaced";
    }

    cx.render(rsx! {
        p {
            class: "{classes}",
            &cx.props.children
        }
    })
}
