//! Dioxus Bulma example
//!
mod content;
mod pages;
use pages::*;

use bulma::{
    dioxus_router::{Route, Router},
    prelude::*,
};

fn main() {
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
    use_shared_state_provider(cx, || GlobalState::default());
    // use_init_signal_rt(cx);

    cx.render(rsx! {
        style { "{get_bulma_css()}" },
        Router {
            Header {},
            Route {
                to: "/home"
                HomePage{}
            },
            Route {
                to: "/posts"
                PostsPage{}
            },
            Route {
                to: "/"
                AuthorsPage{}
            },
            Route {
                to: ""
                NotFoundPage{}
            },
            Footer {}
        }
    })
}

#[allow(non_snake_case)]
fn NotFoundPage(cx: Scope) -> Element {
    cx.render(rsx! { div { "Page not found" } })
}
