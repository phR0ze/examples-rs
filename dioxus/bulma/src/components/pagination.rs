//! Provides a responsive, usable and flexible pagination component
//!
use dioxus::prelude::*;

#[allow(non_snake_case)]
#[derive(Props)]
pub struct PaginationProps<'a> {
    children: Element<'a>,
}

/// Pagination is the parent of all the pagination components and must be used
/// as the outside container for them to work correctly
///
/// ### Properties
/// * `children: Element<'a>` is all of the child elements that you can add
#[allow(non_snake_case)]
pub fn Pagination<'a>(cx: Scope<'a, PaginationProps<'a>>) -> Element {
    cx.render(rsx! {
        nav {
            class: "pagination",
            &cx.props.children
        }
    })
}

#[allow(non_snake_case)]
#[derive(Props)]
pub struct PaginationNextProps<'a> {
    children: Element<'a>,
}

/// PaginationPrev provides incremental navigation forward in the pagination list
/// * WARNING: It is to be used as a child item inside the Pagination parent
///
/// ### Properties
/// * `children: Element<'a>` is all of the child elements that you can add
#[allow(non_snake_case)]
pub fn PaginationNext<'a>(cx: Scope<'a, PaginationNextProps<'a>>) -> Element {
    cx.render(rsx! {
        a {
            class: "pagination-next",
            &cx.props.children
        }
    })
}

#[allow(non_snake_case)]
#[derive(Props)]
pub struct PaginationPrevProps<'a> {
    children: Element<'a>,
}

/// PaginationPrev provides incremental navigation backward in the pagination list
/// * WARNING: It is to be used as a child item inside the Pagination parent
///
/// ### Properties
/// * `children: Element<'a>` is all of the child elements that you can add
#[allow(non_snake_case)]
pub fn PaginationPrev<'a>(cx: Scope<'a, PaginationPrevProps<'a>>) -> Element {
    cx.render(rsx! {
        a {
            class: "pagination-next",
            &cx.props.children
        }
    })
}
