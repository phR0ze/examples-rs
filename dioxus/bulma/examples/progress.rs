use bulma::{components::*, elements::*, layouts::*, prelude::*};

static PROGRESS1: AtomRef<Progress> = |_| Progress::default();
static PROGRESS2: AtomRef<Progress> = |_| Progress::default();
static PROGRESS3: AtomRef<Progress> = |_| Progress::default();
static PROGRESS4: AtomRef<Progress> = |_| Progress::default();

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
    fermi::use_init_atom_root(&cx);

    cx.render(rsx! {
        style { "{get_bulma_css()}" },
        Router {
            Header {},
            Route { to: "/", Page1 {} },
            Route { to: "/2", Page2 {} },
        }
    })
}

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

#[allow(non_snake_case)]
fn Page1(cx: Scope) -> Element {
    log::info!("Rendering: Page1");

    cx.render(rsx! {
        ProgressExample1 { id: "1" }
        ProgressExample2 { id: "2" }
        ProgressExample3 { id: "3" }
        ProgressExample4 { id: "4" }
    })
}

#[allow(non_snake_case)]
#[derive(Props)]
pub struct ProgressExampleProps<'a> {
    #[props(!optional)]
    id: &'a str,
}

#[allow(non_snake_case)]
fn ProgressExample1<'a>(cx: Scope<'a, ProgressExampleProps<'a>>) -> Element {
    let state = use_atom_ref(&cx, PROGRESS1);
    let value = state.read().value();
    log::info!("ProgressExample[{}]: render, value: {}", cx.props.id, value);

    // Reset progress on completion
    if state.read().completed() {
        state.write().reset();
    }

    cx.render(rsx! {
        Section { class: "py-2".into(),
            SubTitle { "Progress that will automatically restart when completed" }
            Progress {
                state: PROGRESS1,
                color: Colors::Primary,
            }
            Button {
                color: Colors::Primary,
                onclick: move |_| {
                    state.write().set(value + 0.05)
                },
                "Increment progress {cx.props.id}"
            }
            Button {
                class: "ml-5".into(),
                color: Colors::Warning,
                onclick: move |_| {
                    state.write().reset();
                },
                "Reset progress {cx.props.id}"
            }
            Button {
                class: "ml-5".into(),
                color: Colors::Success,
                onclick: move |_| {
                    state.write().complete();
                },
                "Complete progress {cx.props.id}"
            }
        }
    })
}

#[allow(non_snake_case)]
fn ProgressExample2<'a>(cx: Scope<'a, ProgressExampleProps<'a>>) -> Element {
    let state = use_atom_ref(&cx, PROGRESS2);
    let value = state.read().value();
    log::info!("ProgressExample[{}]: render, value: {}", cx.props.id, value);

    cx.render(rsx! {
        Section { class: "py-2".into(),
            SubTitle { "Progress without any automation" }
            Progress {
                state: PROGRESS2,
                color: Colors::Info,
            }
            Button {
                color: Colors::Info,
                onclick: move |_| {
                    state.write().set(value + 0.05)
                },
                "Increment progress {cx.props.id}"
            }
            Button {
                class: "ml-5".into(),
                color: Colors::Warning,
                onclick: move |_| {
                    state.write().reset();
                },
                "Reset progress {cx.props.id}"
            }
            Button {
                class: "ml-5".into(),
                color: Colors::Success,
                onclick: move |_| {
                    state.write().complete();
                },
                "Complete progress {cx.props.id}"
            }
        }
    })
}

#[allow(non_snake_case)]
fn ProgressExample3<'a>(cx: Scope<'a, ProgressExampleProps<'a>>) -> Element {
    static ID3: AtomRef<i32> = |_| 3;
    let state = use_atom_ref(&cx, PROGRESS3);
    let id3 = use_atom_ref(&cx, ID3);

    // Restart the progress timer
    if state.read().completed() {
        // Reset the progress state
        state.write().reset();

        // Change id trigger future to be regenerated
        *id3.write_silent() += 1;
        log::debug!("ProgressExample[{}]: reset", id3.read());
    }
    let id = format!("{}", *id3.read());

    log::trace!("ProgressExample[{}]: render", &id);
    cx.render(rsx! {
        Section { class: "py-2".into(),
            SubTitle { "Timed progress restarting every 1 sec" }
            ProgressTimed { id: id.clone(),
                state: PROGRESS3,
                duration: 1000,
                color: Colors::Danger,
            }
            Button {
                class: "ml-5".into(),
                color: Colors::Warning,
                state: ButtonState::Disabled,
                "Reset progress {id}"
            }
            Button {
                class: "ml-5".into(),
                color: Colors::Success,
                state: ButtonState::Disabled,
                "Complete progress {id}"
            }
        }
    })
}

#[allow(non_snake_case)]
fn ProgressExample4<'a>(cx: Scope<'a, ProgressExampleProps<'a>>) -> Element {
    static ID4: AtomRef<i32> = |_| 4;
    let state = use_atom_ref(&cx, PROGRESS4);
    log::trace!("ProgressExample[{}]: render", cx.props.id);
    let id4 = use_atom_ref(&cx, ID4);

    // Calculate conditional button state
    let id = format!("{}", *id4.read());
    let reset_btn = if state.read().completed() {
        rsx! {
            Button {
                class: "ml-5".into(),
                color: Colors::Warning,
                onclick: move |_| {
                    state.write().reset();
                    *id4.write_silent() += 1;
                },
                "Reset progress {id}"
            }
        }
    } else {
        rsx! {
            Button {
                class: "ml-5".into(),
                color: Colors::Warning,
                state: ButtonState::Disabled,
                "Reset progress {id}"
            }
        }
    };

    let id = format!("{}", *id4.read());
    cx.render(rsx! {
        Section { class: "py-2".into(),
            SubTitle { "Timed progress for 5 sec that is restartable" }
            ProgressTimed { id: id.clone(),
                state: PROGRESS4,
                duration: 5000,
                color: Colors::Warning,
            }
            reset_btn
            Button {
                class: "ml-5".into(),
                color: Colors::Success,
                state: ButtonState::Disabled,
                onclick: move |_| {
                    state.write().complete();
                },
                "Complete progress {id}"
            }
        }
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
                        onclick: move |_| use_router(cx).push_route("/", None, None),
                        "Page 1"
                    }
                    NavbarItem {
                        onclick: move |_| use_router(cx).push_route("/2", None, None),
                        "Page 2"
                    }
                }
            }
        }
    })
}
