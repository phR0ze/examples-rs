//! Dioxus Bulma example
//!

use bulma::{
    dioxus_router::{use_router, Router, Route},
    components::{Card, CardContent, CardImage, Navbar},
    elements::{Image, SubTitle, Tag, TagLink, Tags, Title},
    icons::*,
    layouts::{Column, Columns, Container},
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
                //.with_transparent(true)
                //.with_decorations(false)
                .with_inner_size(bulma::dioxus_desktop::LogicalSize::new(1200, 700)),
        ),
    )
}

// UI entry point
#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        style { "{get_bulma_css()}" },
        Router {
            AppHeader {},
            Route {
                to: "/"
                HomePage{}
            },
            Route {
                to: "/posts"
                PostsPage{}
            }, 
            Route {
                to: ""
                NotFoundPage{}
            }, 
            AppFooter {}
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
fn AppFooter(cx: Scope) -> Element {
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
fn AppHeader(cx: Scope) -> Element {
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
        div {
            class: "tile is-ancestor is-vertical",
            div {
                class: "tile is-child hero",
                div {
                    class: "hero-body container pb-0",
                    h1 {
                        class: "title is-1",
                        "Welcome..."
                    },
                    h2 {
                        class: "subtitle",
                        "...to the best yew content"
                    }
                },
                div {
                    class: "tile is-child",
                    figure {
                        class: "image is-3by1",
                        img {
                            src: "https://source.unsplash.com/random/1200x400/?yew"
                        }
                    }
                },
                div {
                    class: "tile is-parent container",
                    div {
                        class: "tile is-parent",
                        div {
                            class: "tile is-child box",
                            p {
                                class: "title",
                                "What are yews?"
                            }
                            p {
                                class: "subtitle",
                                "Everything you need to know!"
                            }
                            div {
                                class: "content",
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
                    div {
                        class: "tile is-parent",
                        div {
                            class: "tile is-child box",
                            p {
                                class: "title",
                                "Who are we?"
                            }
                            div {
                                class: "content",
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
    cx.render(rsx! {
        Container {
            is_fluid: true,
            br {}
            Columns {
                Column {
                    Card {
                        CardImage {
                            Image {
                                src: "https://bulma.io/images/placeholders/1280x960.png".into(),
                                ratio: (16, 9).into(),
                            }
                        }
                        CardContent {
                            Title { "Hello World" }
                            SubTitle { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus nec iaculis mauris." }
                            span {
                                class: "icon-text",
                                span {
                                    class: "is-uppercase has-text-weight-medium is-size-7",
                                    "Read More"
                                }
                                span { 
                                    class: "icon",
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
                Column {
                    Card {
                        CardImage {
                            Image {
                                src: "https://bulma.io/images/placeholders/1280x960.png".into(),
                                ratio: (16, 9).into(),
                            }
                        }
                        CardContent {
                            Title { "Hello World" }
                            SubTitle { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus nec iaculis mauris." }
                        }
                        // Icon {
                        //     IconText { "Read More".into() };
                        //     IconSvg { width: 15, height: 15, src = fa_solid_icons::FaArrowRight }
                        // }
                    }
                }
               Column {
                    Card {
                        CardImage {
                            Image {
                                src: "https://bulma.io/images/placeholders/1280x960.png".into(),
                                ratio: (16, 9).into(),
                            }
                        }
                        CardContent {
                            Title { "Hello World" }
                            SubTitle { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus nec iaculis mauris." }
                        }
                    }
                }
                Column {
                    Card {
                        CardImage {
                            Image {
                                src: "https://bulma.io/images/placeholders/1280x960.png".into(),
                                ratio: (16, 9).into(),
                            }
                        }
                        CardContent {
                            Title { "Hello World" }
                            SubTitle { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus nec iaculis mauris." }
                        }
                    }
                }
            }
                       Columns {
                Column {
                    size: 4,
                    Tags {
                        Tag {
                            color: Colors::Warning,
                            size: Sizes::Medium,
                            "Rust"
                        }
                        Tag {
                            color: Colors::Link,
                            size: Sizes::Medium,
                            "Go"
                        }
                        Tag {
                            color: Colors::Info,
                            size: Sizes::Medium,
                            "Python"
                        }
                        Tag {
                            color: Colors::Danger,
                            size: Sizes::Medium,
                            "Ruby"
                        }
                        Tag {
                            color: Colors::Dark,
                            size: Sizes::Medium,
                            "C++"
                        }
                    }
                }
                Column {
                    size: 3,
                    Tags {
                        Tag {
                            color: Colors::Danger,
                            size: Sizes::Medium,
                            deletable: true,
                            "React"
                        }
                        Tag {
                            color: Colors::Success,
                            size: Sizes::Medium,
                            deletable: true,
                            "Vue"
                        }
                        Tag {
                            color: Colors::Dark,
                            size: Sizes::Medium,
                            deletable: true,
                            "Dioxus"
                        }
                    }
                }
                Column {
                    size: 4,
                    div {
                        class: "field is-grouped is-grouped-multiline",
                        div {
                            class: "control",
                            Tags {
                                addons: true,
                                Tag {
                                    color: Colors::Dark,
                                    size: Sizes::Medium,
                                    "crates.io"
                                }
                                Tag {
                                    color: Colors::Warning,
                                    size: Sizes::Medium,
                                    "v0.2.4"
                                }
                            }
                        }
                        div {
                            class: "control",
                            Tags {
                                addons: true,
                                Tag {
                                    color: Colors::Dark,
                                    size: Sizes::Medium,
                                    "docs"
                                }
                                Tag {
                                    color: Colors::Info,
                                    size: Sizes::Medium,
                                    "latest"
                                }
                            }
                        }
                    }
                }
                Column {
                    size: 1,
                    TagLink {
                        color: Colors::Link,
                        size: Sizes::Medium,
                        onclick: |_| {
                            //toast.write().popup(ToastInfo::simple("clickable tag clicked."));
                        }
                        "Link"
                    }
                }
            }
        }
    })
}
