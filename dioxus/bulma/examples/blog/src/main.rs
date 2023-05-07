//! Dioxus Bulma example
//!
mod content;
mod pages;
use pages::*;

use bulma::{
    components::*,
    dioxus_router::{use_router, Route, Router},
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
    cx.render(rsx! {
        div {
            "Page not found"
        }
    })
}

#[allow(non_snake_case)]
fn Footer(cx: Scope) -> Element {
    cx.render(rsx! {
        footer {
            class: "footer",
            div {
                class: "content has-text-centered",
                "Powered by "
                a { href: "https://dioxuslabs.com/", "Dioxus" }
                " using "
                a { href: "https://bulma.io/", "Bulma" }
                " and images from "
                a { href: "https://unsplash.com", "Unsplash" }
            }
        }
    })
}

#[allow(non_snake_case)]
fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        Navbar {
            color: Colors::Info,
            brand: "https://bulma.io/images/bulma-logo.png".into(),
            div {
                class: "navbar-menu",
                div {
                    class: "navbar-start",
                    a {
                        class: "navbar-item",
                        onclick: move |_| {
                            use_router(cx).replace_route("/", None, None)
                        },
                        "Home"
                    }
                    a {
                        class: "navbar-item",
                        onclick: move |_| {
                            use_router(cx).replace_route("/posts", None, None)
                        },
                        "Posts"
                    }
                    div {
                        class: "navbar-item has-dropdown is-hoverable",
                        div {
                            class: "navbar-link",
                            "More"
                        }
                        div {
                            class: "navbar-dropdown",
                            a {
                                class: "navbar-item",
                                "About"
                            }
                            a {
                                class: "navbar-item",
                                "Meet the authors"
                            }
                        }
                    }
                }
            }
        }
    })
}
