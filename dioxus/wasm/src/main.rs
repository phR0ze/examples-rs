#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_outline_icons::HiBeaker;
use dioxus_free_icons::Icon;
use dioxus_router::*;

mod components;
mod generator;

// Shared state object
struct State {
    count: i32,
}

fn main() {
    // WASM will pull CSS libraries via the index.html
    #[cfg(target_family = "wasm")]
    dioxus_web::launch(App);

    // Conditionally pull in CSS libraries for desktop as they won't be available
    // the same way as WASM through the index.html path
    #[cfg(any(windows, unix))]
    dioxus_desktop::launch_cfg(
        App,
        dioxus_desktop::Config::new()
            .with_window(
                dioxus_desktop::WindowBuilder::new()
                    .with_title("diper")
                    .with_decorations(false)
                    .with_inner_size(dioxus_desktop::LogicalSize::new(300.0, 300.0)),
            )
            .with_custom_head(r#"<link rel="stylesheet" href="./assets/css/tailwind.css">"#.to_string()),
    );
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || State { count: 0 });

    // Resize the window
    #[cfg(any(windows, unix))]
    {
        let win = dioxus_desktop::use_window(cx);
        win.set_inner_size(dioxus_desktop::LogicalSize::new(500.0, 350.0));
    }

    cx.render(rsx! {
        div {
            id: "root" ,
            TitleBar{},
            //NavBar{},

            // // Router
            // Router {
            //     Route { to: "/", Home { } }
            //     Route { to: "/posts", Post {} }
            //     Route { to: "", NotFound {} }
            // },
        }
    })
}

fn TitleBar(cx: Scope) -> Element {
    let win = dioxus_desktop::use_window(cx);

    // The window control icons cause the div to consume a height equal to the icon
    // and the entire width of the window plus padding and margins.
    cx.render(rsx! {
        div {
            id: "titlebar",
            onmousedown: move |_| { win.drag(); },
            Icon {
                fill: "white"
                icon: HiBeaker,
            },
        }
    })
}

fn LoginMessage(cx: Scope) -> Element {
    let msg = "foobar";

    cx.render(rsx! {
        div {
            Icon {
                fill: "white"
                icon: HiBeaker,
            },
            p {
                "{msg}"
            }
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
    let state = use_shared_state::<State>(cx).unwrap();

    cx.render(rsx! {
        div {
            h1 { "Welcome..." }
            p { "...to the best yew content" }
        }
        button {
            class: "bg-gray-200 px-4 py-2 rounded-lg border border-white hover:border-indigo-500 active:scale-95 transition-all",
            onclick: move |_| { state.write().count += 1 },
            "count is {state.read().count}"
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
