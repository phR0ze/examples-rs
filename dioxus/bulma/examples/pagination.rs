use bulma::{
    components::*,
    dioxus_router::{Route, Router},
    layouts::Section,
    prelude::*,
};

static PAGINATION_STATE: fermi::AtomRef<PaginationState> = |_| PaginationState::default();

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

    // By referencing the state object here `App` will receive render events
    // every time the pagination state objecdt changes
    let state = fermi::use_atom_ref(&cx, PAGINATION_STATE);
    let total_pages = 12;

    cx.render(rsx! {
        style { "{get_bulma_css()}" },
        Router {
            Route { to: "/authors/:author", "Authors" },
            Section {
                Pagination { id: "/",
                    state: state,
                    total_pages: total_pages,
                }
            }
        }
    })
}
