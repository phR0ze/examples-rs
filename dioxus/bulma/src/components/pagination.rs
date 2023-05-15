//! Provides a responsive, usable and flexible pagination component
//!
use dioxus::prelude::*;
use fermi::UseAtomRef;
use std::collections::HashMap;

/// Pagination shared state
pub struct PaginationState {
    /// Caching for current pages tracking
    current_pages: HashMap<String, usize>,
}

impl PaginationState {
    /// Get pagination for the given id
    /// * `id: &str` identifier for the pagination e.g. a page route
    pub fn get(&self, id: &str) -> usize {
        let mut value = *self.current_pages.get(id).unwrap_or(&1) as usize;
        if value == 0 {
            value = 1;
        }
        value
    }

    /// Set pagination for the given id
    /// * `id: &str` identifier for the pagination e.g. a page route
    /// * `page: usize` current page to set
    pub fn set(&mut self, id: &str, page: usize) {
        self.current_pages.insert(id.to_string(), page);
    }
}

impl Default for PaginationState {
    fn default() -> Self {
        PaginationState { current_pages: HashMap::new() }
    }
}

#[allow(non_snake_case)]
#[derive(Props)]
pub struct PaginationProps<'a> {
    #[props(!optional)]
    id: &'a str,

    #[props(!optional)]
    total_pages: usize,

    #[props(default = 3)]
    links_per_side: usize,

    #[props(!optional)]
    state: &'a UseAtomRef<PaginationState>,
}

/// Pagination is the parent of all the pagination components and must be used
/// as the outside container for them to work correctly
///
/// ### Warning
/// * must be used inside the Router component to work correctly
///
/// ### Properties
/// * `id: String` id used for pagination lookup
/// * `total_pages: usize` total number of pages to paginate over
/// * `links_per_side: usize` number of links to show to the left and right of the current page
/// * `state: &'a UseAtomRef<GlobalState>` global fermi state reference for tracking
#[allow(non_snake_case)]
pub fn Pagination<'a>(cx: Scope<'a, PaginationProps<'a>>) -> Element {
    let state = cx.props.state;
    let (id1, id2) = (cx.props.id.clone(), cx.props.id.clone());
    let page = state.read().get(&id1);
    let max_links = cx.props.links_per_side;

    let pages_left = page.checked_sub(1).unwrap_or_default();
    let pages_right = cx.props.total_pages - page;
    let mut links_left = max_links.min(pages_left);
    // If not all left links were displayed then add them to the right side
    let links_right = max_links.min(pages_right) + max_links.checked_sub(links_left).unwrap_or_default();
    // If not all right links were displayed then add them to the left side
    links_left = links_left + max_links.checked_sub(links_right).unwrap_or_default();

    let mut prev_css = "".to_string();
    cx.render(rsx! {
        nav { class: "pagination is-right",
            if page == 1 {
                prev_css = "is-disabled".to_string();
            }
            a { class: "pagination-previous {prev_css}",
                onclick: move |_| {
                    if page - 1 > 0 {
                        state.write().set(&id1, page - 1);
                    }
                },
                "Previous"
            }
            a { class: "pagination-next",
                onclick: move |_| {
                    if page + 1 <= cx.props.total_pages {
                        state.write().set(&id2, page + 1);
                    }
                },
                "Next Page"
            }
            ul {
                class: "pagination-list",
                PaginationRange(cx, (1..=pages_left).collect(), links_left, true)
                li { a { class: "pagination-link is-current", "{page}"} }
                PaginationRange(cx, (page+1..=page+pages_right).collect(), links_right, false)
            }
        }
    })
}

/// The range may be to the left or the right of the current page.
/// * `pages` is the page range to potentially display as links for this pagination range
/// * `max` is the max number of pages to display as links for this pagination range
/// * `left` signals the optional ellipsis would be to the left
#[allow(non_snake_case)]
fn PaginationRange<'a>(
    cx: Scope<'a, PaginationProps<'a>>, mut pages: Vec<usize>, max: usize, left: bool,
) -> Element {
    cx.render(if pages.len() > max {
        if left {
            // Split off everything at index max and beyond
            // also taking into account 2 less for the last and ellipsis
            let offset = pages.len().checked_sub(max.checked_sub(2).unwrap_or_default()).unwrap_or_default();
            let right = pages.split_off(offset);
            rsx! {
                PaginationLink(cx, *pages.first().unwrap())
                PaginationEllipsis {}
                for i in (right) {
                    PaginationLink(cx, i)
                }
            }
        } else {
            // Split off everything at index max and beyond
            // also taking into account 2 less for the last and ellipsis
            let right = pages.split_off(max - 2);
            rsx! {
                for i in (pages) { PaginationLink(cx, i) }
                PaginationEllipsis {}
                PaginationLink(cx, *right.last().unwrap())
            }
        }
    } else {
        rsx! {
            for i in (pages) {
                PaginationLink(cx, i)
            }
        }
    })
}

/// PaginationLink provides a clickable pagination button
///
/// ### Properties
/// * `id: &'a str` id used for pagination lookup
#[allow(non_snake_case)]
fn PaginationLink<'a>(cx: Scope<'a, PaginationProps<'a>>, page: usize) -> Element {
    let state = cx.props.state;

    cx.render(rsx! {
        li {
            a { class: "pagination-link",
                onclick: move |_| {
                    state.write().set(&cx.props.id, page);
                },
                format!("{page}")
            }
        }
    })
}

/// PaginationEllipsis provides an ellipsis place holder for pagination buttons
#[allow(non_snake_case)]
fn PaginationEllipsis(cx: Scope) -> Element {
    cx.render(rsx! {
        li {
            span { class: "pagination-ellipsis",
                "..."
            }
        }
    })
}
