use dioxus::prelude::*;

#[allow(non_snake_case)]
#[derive(Props)]
pub struct ImageProps<'a> {
    #[props(optional)]
    size: Option<u8>,

    #[props(optional)]
    ratio: Option<(u8, u8)>,

    #[props(default)]
    is_fullwidth: bool,

    #[props(!optional)]
    src: &'a str,
}

/// Image
#[allow(non_snake_case)]
pub fn Image<'a>(cx: Scope<'a, ImageProps<'a>>) -> Element {
    let mut class = "image".to_string();

    if let Some(size) = cx.props.size {
        class = format!("{class} is-{size}x{size}");
    }

    if let Some(ratio) = cx.props.ratio {
        let a = ratio.0;
        let b = ratio.1;
        class = format!("{class} is-{a}by{b}");
    }

    if cx.props.is_fullwidth {
        class += " is-fullwidth";
    }

    cx.render(rsx! {
        figure {
            class: "{class}",
            img {
                src: "{cx.props.src}"
            }
        }
    })
}
