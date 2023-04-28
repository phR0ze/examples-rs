//! Dioxus Bulma example
//!
pub mod layouts;

// Get BULMA styles as a static str
pub fn get_bulma_css() -> &'static str {
    include_str!("./bulma.min.css")
}

// **********************************************************
// App
// **********************************************************
use dioxus::prelude::*;
use layouts::Container;

fn main() {
    #[cfg(target_family = "wasm")]
    dioxus_web::launch(App);

    #[cfg(any(windows, unix))]
    dioxus_desktop::launch_cfg(
        App,
        dioxus_desktop::Config::new().with_window(
            dioxus_desktop::WindowBuilder::new()
                .with_title("Bulma Example")
                .with_resizable(true)
                //.with_transparent(true)
                //.with_decorations(false)
                .with_inner_size(dioxus_desktop::LogicalSize::new(1200, 700)),
        ),
    )
}

// UI entry point
#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        style { "{get_bulma_css()}" },
        Container {
            is_fluid: true,
            div {
                "hello world"
            }
        }
    })
}
