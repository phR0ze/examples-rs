//! Dioxus Bulma example
//!

use bulma::{
    components::*,
    dioxus_router::{Route, Router},
    layouts::Section,
    prelude::*,
};

static GLOBAL_STATE: fermi::AtomRef<GlobalState> = |_| GlobalState::default();

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
    fermi::use_init_atom_root(&cx);
    let state = fermi::use_atom_ref(&cx, GLOBAL_STATE);
    let total_pages = 12;

    cx.render(rsx! {
        style { "{get_bulma_css()}" },
        Router {
            Route { to: "/authors/:author", "Authors" },
            Section {
                Pagination{
                    state: state,
                    route: "/".into(),
                    total_pages: total_pages,
                }
            }
        }
    })
}
