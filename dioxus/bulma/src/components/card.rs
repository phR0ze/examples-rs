//! A all-around flexible and composable component
//!
use dioxus::prelude::*;

#[allow(non_snake_case)]
#[derive(Props)]
pub struct CardProps<'a> {
    children: Element<'a>,
}

/// Card comprises several elements that you can mix and match
/// * Bulma docs https://bulma.io/documentation/components/card/
///
/// ### Properties
/// * `header: Option<String>` a horizontal content bar with a shadow
/// * `title: Option<String>` is an optional title of the card's content
/// * `content: Element<'a>` is the main part, ideal for text content thanks to its padding
/// * `image: Option<String>` is an optional image for the card
/// * `image_ratio: Option<Ratios>` is an optional ratio to use for the card image
/// * `footer: Element<'a>`
///
/// ### Example
/// ```ignore
/// use bulma::{
///     components::{Card, CardProps},
///     prelude::*,
/// };
/// ```
#[allow(non_snake_case)]
pub fn Card<'a>(cx: Scope<'a, CardProps<'a>>) -> Element {
    //let image_ratio = cx.props.image_ratio.as_ref().map_or("".into(), |x| format!(" is-{}", x));
    // let title = cx.props.title.is_some().then(|| {
    //     cx.render(rsx! {
    //         p {
    //             class: "title",
    //             "{cx.props.title.clone().unwrap()}"
    //         }
    //     })
    // });

    //     cx.render(rsx! {
    //         div {
    //             class: "card",
    //             cx.props.header.as_ref().and_then(|_| cx.render(rsx! {
    //                 footer {
    //                     class: "card-header",
    //                     &cx.props.header
    //                 }
    //             }))
    //         }
    //     })
    // }

    cx.render(rsx! {
        div {
            class: "card",
            &cx.props.children
        }
    })
}

#[allow(non_snake_case)]
#[derive(Props)]
pub struct CardHeaderProps<'a> {
    #[props(optional)]
    title: Option<String>,

    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn CardHeader<'a>(cx: Scope<'a, CardHeaderProps<'a>>) -> Element {
    let title = cx.props.title.is_some().then(|| {
        cx.render(rsx! {
            p {
                class: "card-header-title",
                "{cx.props.title.clone().unwrap()}"
            }
        })
    });

    cx.render(rsx! {
        div {
            class: "card-header",
            title,
            &cx.props.children
        }
    })
}

#[allow(non_snake_case)]
#[derive(Props)]
pub struct CardImageProps<'a> {
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn CardImage<'a>(cx: Scope<'a, CardImageProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "card-image",
            &cx.props.children
        }
    })
}

#[allow(non_snake_case)]
#[derive(Props)]
pub struct CardContentProps<'a> {
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn CardContent<'a>(cx: Scope<'a, CardContentProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "card-content",
            &cx.props.children
        }
    })
}

#[allow(non_snake_case)]
#[derive(Props)]
pub struct CardFooterProps<'a> {
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn CardFooter<'a>(cx: Scope<'a, CardFooterProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "card-footer",
            &cx.props.children
        }
    })
}
