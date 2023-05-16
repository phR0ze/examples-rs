//! Provides a responsive, usable and flexible pagination component
//!
use dioxus::prelude::*;
use fermi::{use_atom_ref, AtomRef};
use std::collections::HashMap;

/// Pagination shared state
struct PaginationMeta {
    pages: Vec<String>,
    current_page: usize,
}

pub struct Pagination {
    state: HashMap<String, PaginationMeta>,
}

impl Pagination {
    /// Set pagination for the given url
    /// * `url: &str` unique page url for pagination
    /// * `pages: Vec<String>` pages to keep track of
    pub fn set(&mut self, url: &str, pages: Vec<String>) {
        self.state.insert(url.to_string(), PaginationMeta { pages, current_page: 1 });
    }

    /// Add or update pagination for the given url
    /// * `url: &str` unique page url for pagination
    /// * `pages: Vec<String>` pages to keep track of
    /// * `current_page: usize` current page
    pub fn set_with_page(&mut self, url: &str, pages: Vec<String>, current_page: usize) {
        self.state.insert(url.to_string(), PaginationMeta { pages, current_page });
    }

    /// Get the current pagination page
    /// * `url: &str` unique page url for pagination
    /// * returns `usize` current page
    pub fn current_page(&self, url: &str) -> usize {
        match self.state.get(url) {
            Some(x) => x.current_page,
            _ => 1, // default
        }
    }

    /// Set the current pagination page
    /// * `url: &str` unique page url for pagination
    /// * `page: usize` current page to set
    pub fn set_current_page(&mut self, url: &str, page: usize) {
        if let Some(ref mut x) = self.state.get_mut(url) {
            x.current_page = page;
        }
    }

    /// Get the total number of pages
    /// * `url: &str` unique page url for pagination
    /// * returns `usize` total pages
    pub fn total_pages(&self, url: &str) -> usize {
        match self.state.get(url) {
            Some(x) => x.pages.len(),
            _ => 0, // default
        }
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Pagination { state: HashMap::new() }
    }
}

#[allow(non_snake_case)]
#[derive(Props, PartialEq)]
pub struct PaginationProps {
    #[props(!optional)]
    url: String,

    #[props(default = 3)]
    links_per_side: usize,

    #[props(!optional)]
    state: fermi::Atom<Pagination>,
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
pub fn Pagination(cx: Scope<PaginationProps>) -> Element {
    let state = fermi::use_atom_state(cx, cx.props.state);

    let page = state.current_page(&cx.props.url);
    let max_links = cx.props.links_per_side;

    let pages_left = page.checked_sub(1).unwrap_or_default();
    let pages_right = state.total_pages(&cx.props.url) - page;
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
                        // state.set_current_page(&cx.props.url, page - 1);
                    }
                },
                "Previous"
            }
            a { class: "pagination-next",
                onclick: move |_| {
                    if page + 1 <= state.total_pages(&cx.props.url) {
                        // state.set_current_page(&cx.props.url, page + 1);
                        log::info!("next page {}", state.current_page(&cx.props.url));
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
fn PaginationRange(cx: Scope<PaginationProps>, mut pages: Vec<usize>, max: usize, left: bool) -> Element {
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
fn PaginationLink(cx: Scope<PaginationProps>, page: usize) -> Element {
    let mut state = fermi::use_atom_state(cx, cx.props.state);

    cx.render(rsx! {
        li {
            a { class: "pagination-link",
                onclick: move |_| {
                    // state.set_current_page(&cx.props.url, page);
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
