//! Dioxus Bulma example
//!

use bulma::{components::*, layouts::Section, prelude::*};

fn main() {
    dioxus_desktop::launch_cfg(
        App,
        dioxus_desktop::Config::new().with_window(
            dioxus_desktop::WindowBuilder::new()
                .with_title("Progress Example")
                .with_resizable(true)
                .with_inner_size(dioxus_desktop::LogicalSize::new(1200, 700)),
        ),
    )
}

// UI entry point
#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    // use_shared_state_provider(cx, || GlobalState::default());

    let total_pages = 12;

    cx.render(rsx! {
        style { "{get_bulma_css()}" },
        Section {
            Pagination{
                route: "/".into(),
                total_pages: total_pages,
            }
        }
    })
}
