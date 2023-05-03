use dioxus::prelude::*;
use dioxus_free_icons::{icons, IconShape};

#[allow(non_snake_case)]
#[derive(Props)]
pub struct IconTextProps<'a> {
    children: Element<'a>,
}

/// IconText provides the ability to combine an icon with text as long as all text inside is
/// wrapped with its own span element
///
/// ### Properties
#[allow(non_snake_case)]
pub fn IconText<'a>(cx: Scope<'a, IconTextProps<'a>>) -> Element {
    cx.render(rsx! {
        span {
            class: "icon-text",
            &cx.props.children
        }
    })
}

#[allow(non_snake_case)]
#[derive(Props)]
pub struct IconProps<'a, T: IconShape> {
    src: T,

    #[props(default = 20)]
    pub height: u32,

    #[props(default = 20)]
    pub width: u32,

    #[props(default = "currentColor")]
    pub fill: &'a str,

    #[props(default = "")]
    pub class: &'a str,
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
/// * `height: u32` the height of the svg element. Defaults to 20
/// * `width: u32` the width of the svg element. Defaults to 20
/// * `fill: &'a str` the color to use for filling the icon. Defaults to "currentColor".
/// * `class: &'a str` additional classes to give the svg element. Defaults to ""
#[allow(non_snake_case)]
pub fn Icon<'a, T: IconShape>(cx: Scope<'a, IconProps<'a, T>>) -> Element {
    cx.render(rsx! {
        span {
            class: "icon",
            i {
                &cx.props.children
            }
        }
    })
}

/// Icon component which generates SVG elements
#[allow(non_snake_case)]
pub fn Icon<'a, T: IconShape>(cx: Scope<'a, IconProps<'a, T>>) -> Element<'a> {
    cx.render(rsx! {
        svg {
            stroke: "currentColor",
            stroke_width: "0",
            class: format_args!("{}", cx.props.class),
            height: format_args!("{}", cx.props.height),
            width: format_args!("{}", cx.props.width),
            view_box: format_args!("{}", cx.props.icon.view_box()),
            xmlns: format_args!("{}", cx.props.icon.xmlns()),
            fill: format_args!("{}", cx.props.fill),
            title {
                "{cx.props.title}"
            }
            cx.props.icon.child_elements()
        }
    })
}

#[allow(non_snake_case)]
#[derive(Props)]
pub struct IconTextProps<'a> {
    children: Element<'a>,
}

/// IconText provides the ability to combine an icon with text as long as all text inside is
/// wrapped with its own span element
///
/// ### Properties
#[allow(non_snake_case)]
pub fn IconText<'a>(cx: Scope<'a, IconTextProps<'a>>) -> Element {
    cx.render(rsx! {
        span {
            class: "icon-text",
            &cx.props.children
        }
    })
}
