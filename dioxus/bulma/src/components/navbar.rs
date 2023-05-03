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
    brand_size: Option<(u16, u16)>,

    #[props(optional)]
    classes: Option<String>,

    children: Element<'a>,
}

/// Navbar
///
/// ### Properties
/// `brand: Option<String>` is an optional Brand image to display in the Navbar on the left
#[allow(non_snake_case)]
pub fn Navbar<'a>(cx: Scope<'a, NavbarProps<'a>>) -> Element {
    // navbar
    let mut navbar_classes = "navbar".to_string();

    if let Some(color) = &cx.props.color {
        navbar_classes = color.append_class(&navbar_classes);
    }

    if let Some(extra_classes) = &cx.props.classes {
        navbar_classes = format!("{navbar_classes} {extra_classes}");
    }

    // navbar-brand
    // -------------------------------------------------------------------------
    let navbar_brand = cx.props.brand.is_some().then(|| {
        let brand_image = if let Some((width, height)) = cx.props.brand_size {
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
            class: "{navbar_classes}",
            navbar_brand,
            &cx.props.children
        }
    })
}
