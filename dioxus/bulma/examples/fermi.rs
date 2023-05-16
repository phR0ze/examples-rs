use bulma::{
    components::*,
    dioxus_router::{Link, Route, Router},
    elements::Title,
    layouts::Section,
    prelude::*,
};
use fermi::AtomRoot;

// Global state
static PAGINATION: fermi::Atom<Pagination> = |_| Pagination::default();

fn main() {
    dioxus_logger::init(log::LevelFilter::Info).expect("failed to init logger");

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

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    log::info!("Rendering: App");

    // Init global state system
    fermi::use_init_atom_root(cx);

    // Load test data
    // let state = use_atom_ref(cx, PAGINATION);
    // state.write_silent().set("/posts", (1..=12).map(|x| x.to_string()).collect::<Vec<String>>());
    // state.write_silent().set("/authors", (1..=22).map(|x| x.to_string()).collect::<Vec<String>>());

    cx.render(rsx! {
        style { "{get_bulma_css()}" },
        Router {
            Header {},

            div {
                Link { to: "/posts", Posts {} },
            }
            Route { to: "/", AtomRootExample {} },
            // Route { to: "/authors", Authors {} },
            // Link { to: "/posts", Posts {} },
            // Link { to: "/authors", Authors {} },
        }
    })
}

#[allow(non_snake_case)]
pub fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        Navbar {
            color: Colors::Primary,
            div {
                class: "navbar-menu",
                div {
                    class: "navbar-start",
                    a {
                        class: "navbar-item",
                        onclick: move |_| {
                            use_router(cx).push_route("/", None, None)
                        },
                        "Home"
                    }
                    a {
                        class: "navbar-item",
                        onclick: move |_| {
                            use_router(cx).push_route("/posts", None, None)
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
                                onclick: move |_| {
                                    use_router(cx).push_route("/authors", None, None)
                                },
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
fn AtomRootExample(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "no comprendo"
        }
    })
}

#[allow(non_snake_case)]
fn Posts(cx: Scope) -> Element {
    log::info!("Rendering: Posts");

    // Example content loading
    let future = use_future(cx, (), |_| async move {
        // let state = fermi::use_atom_root(cx);
        // let pagination = state.read(PAGINATION);
        // pagination.set("/posts", (1..=12).map(|x| x.to_string()).collect::<Vec<String>>());
    });

    // Render page
    match future.value() {
        Some(_) => {
            let url = "/posts".to_string();
            let state = fermi::use_atom_state(cx, PAGINATION);
            let page = state.current_page(&url);
            cx.render(rsx! {
                Section {
                    Title { "Page: {page}" }
                    Pagination { url: "/posts".to_string(),
                        state: PAGINATION,
                    }
                }
            })
        },
        _ => cx.render(rsx! { div { "loading posts..."} }),
    }
}

#[allow(non_snake_case)]
fn Authors(cx: Scope) -> Element {
    log::info!("Rendering: Authors");

    cx.render(rsx! {
        Section {
            Pagination { url: "/authors".to_string(),
                state: PAGINATION,
            }
        }
    })
}
