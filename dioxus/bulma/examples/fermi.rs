use bulma::{
    components::*,
    dioxus_router::{Link, Route, Router},
    elements::*,
    layouts::*,
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

    cx.render(rsx! {
        style { "{get_bulma_css()}" },
        Router {
            Header {},
            Route { to: "/", Page1 {} },
            Route { to: "/2", Page2 {} },
            Route { to: "/3", Page3 {} },
        }
    })
}

/// Use `fermi::use_atom_state` for primitive objects with no methods. This call will register
/// the calling component to receive render events each time the state changes. You must use the
/// fermi::Atom<T> static construction which can't be used with reference values.
#[allow(non_snake_case)]
fn Page1(cx: Scope) -> Element {
    log::info!("Rendering: Page1");
    static COUNT: fermi::Atom<i32> = |_| 0;
    let mut count = fermi::use_atom_state(cx, COUNT);

    cx.render(rsx! {
        Section {
            Title { "Page 1"}
            SubTitle { "Count: {count}" }
            Button {
                color: Colors::Danger,
                onclick: move |_| { count -= 1 },
                "-"
            }
            Button { class: "ml-1",
                color: Colors::Primary,
                onclick: move |_| { count += 1 },
                "+"
            }
            Button { class: "ml-1",
                color: Colors::Primary,
                onclick: move |_| { count.set(5) },
                "set(5)"
            }
            Button { class: "ml-1",
                color: Colors::Primary,
                onclick: move |_| { count.modify(|x| x + 2) },
                "modify(|x| x + 2)"
            }
        }
    })
}

/// Use `fermi::use_atom_ref` for complex objects that have methods and fields. This provides
/// interior mutability with RefCell using the `read` and `write` functions. This requires the
/// use of `fermi::AtomRef<T> to initialize your object`
#[allow(non_snake_case)]
fn Page2(cx: Scope) -> Element {
    log::info!("Rendering: Page2");
    static COUNTS: fermi::AtomRef<Vec<i32>> = |_| vec![0];
    let counts = fermi::use_atom_ref(cx, COUNTS);
    let count = *counts.read().last().unwrap() + 1;
    let str_cnts = format!("{:?}", counts.read());

    cx.render(rsx! {
        Section {
            Title { "Page 2"}
            SubTitle { "Counts: {str_cnts}" }
            Button {
                color: Colors::Danger,
                onclick: move |_| { counts.write().pop(); },
                "-"
            }
            Button { class: "ml-1",
                color: Colors::Primary,
                onclick: move |_| { counts.write().push(count) },
                "+"
            }
        }
    })
}

/// Use the `use_future` in conjunction with `use_atom_ref` to load data and persist it.
#[allow(non_snake_case)]
fn Page3(cx: Scope) -> Element {
    log::info!("Rendering: Page3");

    // Setup state flags. You need to clone them to get a non reference type
    static LOAD: fermi::AtomRef<bool> = |_| false;
    static FAIL: fermi::AtomRef<bool> = |_| false;
    let load = fermi::use_atom_ref(cx, LOAD).clone();
    let fail = fermi::use_atom_ref(cx, FAIL).clone();
    let load2 = load.clone();
    let fail2 = fail.clone();

    // Setup persistent data store
    static COUNTS: fermi::AtomRef<Vec<i32>> = |_| vec![0];
    let counts = fermi::use_atom_ref(cx, COUNTS).clone();
    let counts2 = counts.clone();

    // Mimic loading state
    let future: &UseFuture<Result<(), ()>> = use_future(cx, (), |_| async move {
        to_owned![load, fail, counts];
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
            if *load.read() || *fail.read() {
                break;
            }
        }
        if *load.read() {
            // Load data
            counts.write().extend([1, 2, 3, 4, 5]);
            Ok(())
        } else {
            Err(())
        }
    });

    let str_cnts = format!("{:?}", counts2.read());

    cx.render(match future.value() {
        Some(Ok(_)) => rsx! {
            Section {
                Title { "Success loading content!" }
                SubTitle { "Counts: {str_cnts}" }
            }
        },
        Some(Err(_)) => rsx! {
            Section {
                Title { "Failed loading content!" }
            }
        },
        None => rsx! {
            Section {
                Title { "loading content..."}
                SubTitle { "Counts: {str_cnts}" }
                Button {
                    color: Colors::Danger,
                    onclick: move |_| { *fail2.write() = true; },
                    "FAIL"
                }
                Button { class: "ml-1",
                    color: Colors::Primary,
                    onclick: move |_| { *load2.write() = true; },
                    "LOAD"
                }
            }
        },
    })
}

#[allow(non_snake_case)]
pub fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        Navbar {
            color: Colors::Primary,
            NavbarMenu {
                NavbarStart {
                    NavbarItem {
                        onclick: move |_| { use_router(cx).push_route("/", None, None) },
                        "Page 1"
                    }
                    NavbarItem {
                        onclick: move |_| { use_router(cx).push_route("/2", None, None) },
                        "Page 2"
                    }
                    NavbarItem {
                        onclick: move |_| { use_router(cx).push_route("/3", None, None) },
                        "Page 3"
                    }
                }
            }
        }
    })
}
