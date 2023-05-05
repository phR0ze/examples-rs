
//! Dioxus Bulma example
//!
mod assets;
use crate::assets as model;
use assets::Generated;
use once_cell::sync::Lazy;

use bulma::{
    dioxus_router::{use_router, Router, Route},
    components::*,
    elements::*,
    icons::*,
    layouts::*,
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

// UI entry point
#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || GlobalState::default());

    cx.render(rsx! {
        style { "{get_bulma_css()}" },
        Router {
            Header {},
            Route {
                to: "/home"
                HomePage{}
            },
            Route {
                to: "/"
                PostsPage{}
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

#[allow(non_snake_case)]
fn HomePage(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "tile is-ancestor is-vertical",
            div { class: "tile is-child hero",
                div { class: "hero-body container pb-5",
                    h1 { class: "title is-1",
                        "Welcome..."
                    },
                    h2 { class: "subtitle",
                        "...to the best yew content"
                    }
                },
                div { class: "tile is-child",
                    figure { class: "image is-3by1",
                        img { src: "https://source.unsplash.com/random/1200x400/?yew" }
                    }
                },
                div { class: "tile is-parent container",
                    div { class: "tile is-parent",
                        div { class: "tile is-child box",
                            p { class: "title",
                                "What are yews?"
                            }
                            p { class: "subtitle",
                                "Everything you need to know!"
                            }
                            div { class: "content",
                                r#"
                                A yew is a small to medium-sized evergreen tree, growing 10 to 20 metres tall, with a trunk up to 2 metres in diameter.
                                The bark is thin, scaly brown, coming off in small flakes aligned with the stem.
                                The leaves are flat, dark green, 1 to 4 centimetres long and 2 to 3 millimetres broad, arranged spirally on the stem,
                                but with the leaf bases twisted to align the leaves in two flat rows either side of the stem,
                                except on erect leading shoots where the spiral arrangement is more obvious.
                                The leaves are poisonous.
                                "#
                            }
                        }
                    }
                    div { class: "tile is-parent",
                        div { class: "tile is-child box",
                            p { class: "title",
                                "Who are we?"
                            }
                            div { class: "content",
                                "We're a small team of just 2"
                                sup {
                                    "64"
                                }
                                " members working tirelessly to bring you the low-effort yes conent we all desperately crave."
                                br {}
                                r#"
                                We put a ton of effort into fact-checking our posts.
                                Some say they read like a Wikipedia article - what a compliment!
                                "#
                            }
                        }
                    }
                }
            }
        }
    })
}

#[allow(non_snake_case)]
fn PostsPage(cx: Scope) -> Element {
    let state = use_shared_state::<GlobalState>(cx)?;

    let per_page = 9;
    let cols = 3;
    let per_col = per_page/cols;
    let total_pages = 12;

    // Generate posts
    let start_seed = state.read().pagination.get_current_page("/posts") * per_page;
    let mut posts = (0..per_page).map(|seed_offset| {
        model::PostMeta::generate_from_seed((start_seed + seed_offset) as u64)
    });

    cx.render(rsx! {
        Section {
            Container {
                is_fluid: true,
                Title { "Posts" }
                SubTitle { "All of our quality writing in one place!" }
                Columns {
                    for _ in (1..=cols) {
                        Column {
                            List {
                                for post in posts.by_ref().take(per_col) {
                                    Post {
                                        img_src: post.image_url,
                                    }
                                }
                            }
                        }
                    }
                }
                Pagination{
                    route: "/posts".into(),
                    total_pages: total_pages,
                }
            }
        }
    })
}

#[allow(non_snake_case)]
#[derive(PartialEq, Props)]
pub struct PostProps {
    #[props(!optional)]
    img_src: String,
}

#[allow(non_snake_case)]
pub fn Post<'a>(cx: Scope<'a, PostProps>) -> Element {
    cx.render(rsx! {
        ListItem { class: "mb-5".into(),
            Card {
                CardImage {
                    Image { ratio: (2, 1).into(),
                        src: &cx.props.img_src,
                    }
                }
                CardContent {
                    Title { "Hello World" }
                    SubTitle { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus nec iaculis mauris." }
                    span { class: "icon-text",
                        span { class: "is-uppercase has-text-weight-medium is-size-7",
                            "Read More"
                        }
                        span { class: "icon",
                            Icon {
                                width: 15,
                                height: 15,
                                icon: fa_solid_icons::FaArrowRight,
                            }
                        }
                    }
                }                
            }
        }
    })
}
