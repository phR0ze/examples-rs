use dioxus::prelude::*;

#[allow(non_snake_case)]
#[derive(Props)]
pub struct IconProps<'a> {
    #[props(optional)]
    size: Option<u8>,

    #[props(optional)]
    ratio: Option<(u8, u8)>,

    #[props(default)]
    is_fullwidth: bool,

    #[props(!optional)]
    src: &'a str,
}

/// Icon is a container for any type of icon font. Because the icons can take a few seconds to load,
/// and because you want control over the space the icons will take, you can use Icon as a reliable
/// container that will prevent the page from jumping on page load.
///
/// By default the Icon container will take up exactly 1.5rem x 1.5rem. The icon itself is sized
/// accordingly to the icon library you're using. For example Font Awesome 5 icons will inherit
/// the font size.
///
/// ### Properties
#[allow(non_snake_case)]
pub fn Icon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let mut classes = "icon".to_string();

    // if let Some(size) = cx.props.size {
    //     classes = format!("{classes} is-{size}x{size}");
    // }

    // if let Some(ratio) = cx.props.ratio {
    //     let a = ratio.0;
    //     let b = ratio.1;
    //     classes = format!("{classes} is-{a}by{b}");
    // }

    // if cx.props.is_fullwidth {
    //     classes += " is-fullwidth";
    // }

    cx.render(rsx! {
        span {
            class: "{classes}",
            i {
                src: "{cx.props.src}"
            }
        }
    })
}
