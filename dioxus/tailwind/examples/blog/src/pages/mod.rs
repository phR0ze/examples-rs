mod author;
mod authors;
mod home;
// mod post;
mod posts;
pub use author::*;
pub use authors::*;
pub use home::*;
// pub use post::*;
pub use posts::*;

use bulma::{components::*, prelude::*};

#[allow(non_snake_case)]
pub fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        Navbar {
            color: Colors::Info,
            brand: "https://bulma.io/images/bulma-logo.png".into(),
            NavbarMenu {
                NavbarStart {
                    NavbarItem {
                        onclick: move |_| use_router(cx).push_route("/", None, None),
                        "Home"
                    }
                    NavbarItem {
                        onclick: move |_| use_router(cx).push_route("/posts", None, None),
                        "Posts"
                    }
                    NavbarDropdown {
                        title: "More".into(),
                        NavbarItem {
                            onclick: move |_| use_router(cx).push_route("/authors", None, None),
                            "Meet the authors"
                        }
                    }
                }
            }
        }
    })
}

#[allow(non_snake_case)]
pub fn Footer(cx: Scope) -> Element {
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
