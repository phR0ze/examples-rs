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

/// Simple headings to add depth to your page
///
/// ### Properties
/// * `size: Option<u8>` title size supports sizes 1-6
/// * `is_spaced: bool` allows you to maintain the normal spacing between titles and subtitles
/// * `children: Element<'a>` any other child objects to be contained inside this object
#[allow(non_snake_case)]
pub fn Title<'a>(cx: Scope<'a, TitleProps<'a>>) -> Element {
    let mut class = "title".to_string();

    if let Some(size) = cx.props.size {
        // Bulma only has 1-6 sizes
        if size > 0 && size < 7 {
            class = format!("{class} is-{size}");
        }
    }

    if cx.props.is_spaced {
        class += " is-spaced";
    }

    cx.render(rsx! {
        p {
            class: "{class}",
            &cx.props.children
        }
    })
}

/// Simple headings to add depth to your page
///
/// ### Properties
/// * `size: Option<u8>` sub title size supports sizes 1-6
/// * `is_spaced: bool` allows you to maintain the normal spacing between titles and subtitles
/// * `children: Element<'a>` any other child objects to be contained inside this object
#[allow(non_snake_case)]
pub fn SubTitle<'a>(cx: Scope<'a, TitleProps<'a>>) -> Element {
    let mut class = "subtitle".to_string();

    if let Some(size) = cx.props.size {
        // Bulma only has 1-6 sizes
        if size > 0 && size < 7 {
            class = format!("{class} is-{size}");
        }
    }

    if cx.props.is_spaced {
        class += " is-spaced";
    }

    cx.render(rsx! {
        p {
            class: "{class}",
            &cx.props.children
        }
    })
}
