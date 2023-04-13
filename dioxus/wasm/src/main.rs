#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::*;
use dioxus_web::Config;

mod components;
mod generator;
use components::about::*;
use components::card::*;

fn main() {
    dioxus_web::launch_cfg(
        App,
        Config::new(), //.with_custom_head("<script src=\"https://cdn.tailwindcss.com\"></script>".to_string()),
    );
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            NavBar{}
            Route { to: "/", Home {} }
            Route { to: "/posts", Post {} }
            Route { to: "", NotFound {} }
        }
    })
}

fn NavBar(cx: Scope) -> Element {
    cx.render(rsx! {
        ul {
            Link { to: "/",  li { "Home" } }
            Link { to: "/posts",  li { "Posts" } }
            Link { to: "/authors", li { "More" } }
        }
    })
}

fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            h1 { "Welcome..." }
            p { "...to the best yew content" }
        }

        // View info tiles
        div {
            p { "What are yews?" }
            p { "Everything you need to know!" }
            div {
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
    })
}

fn Post(cx: Scope) -> Element {
    let post = dioxus_router::use_route(cx).last_segment().unwrap();

    cx.render(rsx! {
        div {
            h1 { "Reading blog page: {post}" }
            p { "example blog post" }
        }
    })
}

fn NotFound(cx: Scope) -> Element {
    cx.render(rsx! {
        p { "Err 404 Route Not Found" }
    })
}
