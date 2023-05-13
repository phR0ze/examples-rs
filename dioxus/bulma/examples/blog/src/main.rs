//! Dioxus Bulma example
//!
mod content;
mod pages;

use bulma::{
    dioxus_router::{Route, Router},
    prelude::*,
};

static PAGINATION_STATE: fermi::AtomRef<PaginationState> = |_| PaginationState::default();

struct AppRoutes<'a> {
    pub root: &'a str,
    pub posts: &'a str,
    pub authors: &'a str,
}
static ROUTES: AppRoutes = AppRoutes { root: "/", posts: "/posts", authors: "/authors" };

fn main() {
    dioxus_logger::init(log::LevelFilter::Info).expect("failed to init logger");

    #[cfg(target_family = "wasm")]
    bulma::dioxus_web::launch(App);

    #[cfg(any(windows, unix))]
    bulma::dioxus_desktop::launch_cfg(
        App,
        bulma::dioxus_desktop::Config::new().with_window(
            bulma::dioxus_desktop::WindowBuilder::new()
                .with_title("Bulma Example")
                .with_resizable(true)
                .with_inner_size(bulma::dioxus_desktop::LogicalSize::new(1200, 700)),
        ),
    )
}

// UI entry point that will only get called once on startup
#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    fermi::use_init_atom_root(&cx);

    cx.render(rsx! {
        style { "{get_bulma_css()}" },
        Router {
            pages::Header {},
            Route { to: ROUTES.root, pages::Home{} },
            Route { to: ROUTES.posts, pages::Posts{} },
            // Route { to: "/posts/:post", pages::Post{} },
            Route { to: ROUTES.authors, pages::Authors {} },
            Route { to: "/authors/:author", pages::Author{} },
            Route { to: "" NotFoundPage{} },
            pages::Footer {}
        }
    })
}

#[allow(non_snake_case)]
fn NotFoundPage(cx: Scope) -> Element {
    cx.render(rsx! { p { "Page not found" } })
}
