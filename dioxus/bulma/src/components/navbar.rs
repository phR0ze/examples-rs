//! A responsive horizontal navbar that can support images, links, buttons and dropdowns
//!
use crate::utils::*;
use dioxus::prelude::*;

#[allow(non_snake_case)]
#[derive(Props)]
pub struct NavbarProps<'a> {
    #[props(optional)]
    color: Option<Colors>,

    #[props(optional)]
    brand: Option<String>,

    #[props(optional)]
    brand_image_size: Option<(u16, u16)>,

    #[props(optional)]
    class: Option<String>,

    children: Element<'a>,
}

/// Navbar
///
/// ### Properties
/// * `color: Option<String>` optional color to use
/// * `brand: Option<String>` is an optional Brand image to display in the Navbar on the left
#[allow(non_snake_case)]
pub fn Navbar<'a>(cx: Scope<'a, NavbarProps<'a>>) -> Element {
    // navbar
    let mut navbar_class = "navbar".to_string();

    if let Some(color) = &cx.props.color {
        navbar_class = color.append_class(&navbar_class);
    }

    if let Some(extra_class) = &cx.props.class {
        navbar_class = format!("{navbar_class} {extra_class}");
    }

    // navbar-brand
    // -------------------------------------------------------------------------
    let navbar_brand = cx.props.brand.is_some().then(|| {
        let brand_image = if let Some((width, height)) = cx.props.brand_image_size {
            cx.render(rsx! {
                img { width: "{width}", height: "{height}", src: "{cx.props.brand.clone().unwrap()}" }
            })
        } else {
            cx.render(rsx! {
                img { src: "{cx.props.brand.clone().unwrap()}" }
            })
        };

        cx.render(rsx! {
            div {
                class: "navbar-brand",
                span {
                    class: "navbar-item",
                    brand_image,
                }
            }
        })
    });

    cx.render(rsx! {
        nav {
            class: "{navbar_class}",
            navbar_brand,
            &cx.props.children
        }
    })
}

#[allow(non_snake_case)]
#[derive(Props)]
pub struct NavbarMenuProps<'a> {
    children: Element<'a>,
}

/// NavbarMenu
///
/// ### Properties
#[allow(non_snake_case)]
pub fn NavbarMenu<'a>(cx: Scope<'a, NavbarMenuProps<'a>>) -> Element {
    let class = "navbar-menu".to_string();

    cx.render(rsx! {
        div {
            class: "{class}",
            &cx.props.children
        }
    })
}

#[allow(non_snake_case)]
#[derive(Props)]
pub struct NavbarStartProps<'a> {
    children: Element<'a>,
}

/// NavbarStart
///
/// ### Properties
#[allow(non_snake_case)]
pub fn NavbarStart<'a>(cx: Scope<'a, NavbarStartProps<'a>>) -> Element {
    let class = "navbar-start".to_string();

    cx.render(rsx! {
        div {
            class: "{class}",
            &cx.props.children
        }
    })
}

#[allow(non_snake_case)]
#[derive(Props)]
pub struct NavbarItemProps<'a> {
    #[props(default)]
    onclick: EventHandler<'a, MouseEvent>,

    children: Element<'a>,
}

/// NavbarItem
///
/// ### Properties
#[allow(non_snake_case)]
pub fn NavbarItem<'a>(cx: Scope<'a, NavbarItemProps<'a>>) -> Element {
    let class = "navbar-item".to_string();

    cx.render(rsx! {
        a {
            class: "{class}",
            onclick: move |evt| cx.props.onclick.call(evt),
            &cx.props.children
        }
    })
}

#[allow(non_snake_case)]
#[derive(Props)]
pub struct NavbarDropdownProps<'a> {
    #[props(!optional)]
    title: String,

    children: Element<'a>,
}

/// NavbarDropdown
///
/// ### Properties
#[allow(non_snake_case)]
pub fn NavbarDropdown<'a>(cx: Scope<'a, NavbarDropdownProps<'a>>) -> Element {
    let class = "navbar-item has-dropdown is-hoverable".to_string();

    cx.render(rsx! {
        div {
            class: "{class}",
            div {
                class: "navbar-link",
                "{cx.props.title}",
            }
            div {
                class: "navbar-dropdown",
                &cx.props.children
            }
        }
    })
}
