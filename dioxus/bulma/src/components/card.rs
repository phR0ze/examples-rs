//! A all-around flexible and composable component
//!
use dioxus::prelude::*;

#[allow(non_snake_case)]
#[derive(Props)]
pub struct CardProps<'a> {
    #[props(default)]
    header: Element<'a>,

    #[props(default)]
    content: Element<'a>,

    #[props(default)]
    image: Element<'a>,

    #[props(default)]
    footer: Element<'a>,
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

    cx.render(rsx! {
        div {
            class: "card",
            cx.props.header.as_ref().and_then(|_| cx.render(rsx! {
                footer {
                    class: "card-header",
                    &cx.props.header
                }
            }))
            cx.props.image.as_ref().and_then(|_| cx.render(rsx! {
                div {
                    class: "card-image",
                    &cx.props.image
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
