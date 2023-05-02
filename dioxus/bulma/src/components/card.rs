//! A all-around flexible and composable component
//!
use dioxus::prelude::*;

use crate::utils::*;

#[allow(non_snake_case)]
#[derive(Props)]
pub struct CardProps<'a> {
    #[props(optional)]
    title: Option<String>,

    #[props(default)]
    title_first: bool,

    #[props(default)]
    content: Element<'a>,

    #[props(optional)]
    image: Option<String>,

    #[props(optional)]
    image_ratio: Option<Ratios>,

    #[props(default)]
    footer: Element<'a>,
}

/// Card comprises several elements that you can mix and match
/// * Bulma docs https://bulma.io/documentation/components/card/
///
/// ### Properties
/// * `title: Option<String>` is an optional title of the card
/// * `title_first: bool` specifies that the title should come first rather than the image
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
    let image_ratio = cx.props.image_ratio.as_ref().map_or("".into(), |x| format!(" is-{}", x));

    cx.render(rsx! {
        div {
            class: "card",
            cx.props.title.as_ref().and_then(|_| cx.render(rsx! {
                header {
                    class: "card-header",
                    p {
                        class: "card-header-title",
                        "{cx.props.title.clone().unwrap()}"
                    }
                }
            }))
            cx.props.image.as_ref().and_then(|_| cx.render(rsx! {
                div {
                    class: "card-image",
                    figure {
                        class: "image {image_ratio}",
                        img {
                            src: "{cx.props.image.clone().unwrap()}"
                        }
                    }
                }
            }))
            cx.props.content.as_ref().and_then(|_| cx.render(rsx! {
                div {
                    class: "card-content",
                    &cx.props.content
                }
            }))
            cx.props.footer.as_ref().and_then(|_| cx.render(rsx! {
                footer {
                    class: "card-footer",
                    &cx.props.footer
                }
            }))
        }
    })
}

#[allow(non_snake_case)]
#[derive(PartialEq, Eq, Props)]
pub struct CardHeaderProps {
    #[props(default)]
    title: String,
    //#[props(default)]
    //icon: Element<'a>,
}

#[allow(non_snake_case)]
pub fn CardHeader(cx: Scope<CardHeaderProps>) -> Element {
    cx.render(rsx! {
        p {
            class: "card-header-title",
            "{cx.props.title}"
        }
    })
}
