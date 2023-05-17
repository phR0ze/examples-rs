use bulma::{components::*, elements::*, layouts::*, prelude::*};
use rand::Rng;

fn main() {
    dioxus_logger::init(log::LevelFilter::Info).expect("failed to init logger");

    dioxus_desktop::launch_cfg(
        App,
        dioxus_desktop::Config::new().with_window(
            dioxus_desktop::WindowBuilder::new()
                .with_resizable(true)
                .with_inner_size(dioxus_desktop::LogicalSize::new(1200, 700)),
        ),
    )
}

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    log::info!("Rendering: App");

    // Init global state system
    use_init_atom_root(cx);

    cx.render(rsx! {
        style { "{get_bulma_css()}" },
        Router {
            Header {},
            Route { to: "/", Posts {} },
            Route { to: "/posts", Posts {} },
            Route { to: "/authors", Authors {} },
        }
    })
}

// Persistent pagination storage
static PAGINATION: AtomRef<Pagination> = |_| Pagination::default();

#[allow(non_snake_case)]
fn Posts(cx: Scope) -> Element {
    log::info!("Rendering: Posts");

    // Setup persistent pagination storage
    let state = use_atom_ref(cx, PAGINATION);

    // Example content loading
    let future = use_future(cx, (), |_| {
        to_owned![state];
        async move {
            // Sleep to simulate loading time
            tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;

            // Generate some random total page count
            let pages = rand::thread_rng().gen_range(10..100);
            state.write().set("/posts", (1..=pages).map(|x| x.to_string()).collect::<Vec<String>>());
        }
    });

    // Render page
    match future.value() {
        Some(_) => {
            let url = "/posts".to_string();
            let page = state.read().current_page(&url);
            cx.render(rsx! {
                Section {
                    Title { "Posts" }
                    SubTitle { "Page: {page}" }
                    Pagination { url: "/posts".to_string(),
                        state: state,
                    }
                }
            })
        },
        _ => cx.render(rsx! {
            Section {
                Title { "Posts" }
                SubTitle { "loading posts..." }
            }
        }),
    }
}

#[allow(non_snake_case)]
fn Authors(cx: Scope) -> Element {
    let route = "authors".to_string();
    log::info!("Rendering: {}", &route);

    // Setup persistent pagination storage
    let state = use_atom_ref(cx, PAGINATION);

    // Example content loading
    let future = use_future(cx, (), |_| {
        to_owned![state, route];
        async move {
            // Sleep to simulate loading time
            tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;

            // Generate some random total page count
            let pages = rand::thread_rng().gen_range(10..100);
            state.write().set(&route, (1..=pages).map(|x| x.to_string()).collect::<Vec<String>>());
        }
    });

    // Render page
    match future.value() {
        Some(_) => {
            let page = state.read().current_page(&route);
            cx.render(rsx! {
                Section {
                    Title { "Authors" }
                    SubTitle { "Page: {page}" }
                    Pagination { url: route,
                        state: state,
                    }
                }
            })
        },
        _ => cx.render(rsx! {
            Section {
                Title { "Authors" }
                SubTitle { "loading authors..." }
            }
        }),
    }
}

#[allow(non_snake_case)]
pub fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        Navbar {
            color: Colors::Primary,
            NavbarMenu {
                NavbarStart {
                    NavbarItem {
                        onclick: move |_| { use_router(cx).push_route("/posts", None, None) },
                        "Posts"
                    }
                    NavbarItem {
                        onclick: move |_| { use_router(cx).push_route("/authors", None, None) },
                        "Authors"
                    }
                }
            }
        }
    })
}
