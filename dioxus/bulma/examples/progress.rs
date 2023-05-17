use bulma::{elements::*, layouts::*, prelude::*};

static PROGRESS1: AtomRef<Progress> = |_| Progress::default();
static PROGRESS2: AtomRef<Progress> = |_| Progress::default();
static PROGRESS3: AtomRef<Progress> = |_| Progress::default();
static PROGRESS4: AtomRef<Progress> = |_| Progress::default();

fn main() {
    dioxus_logger::init(log::LevelFilter::Debug).expect("failed to init logger");

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
    log::debug!("App: render");
    fermi::use_init_atom_root(&cx);

    // When the ProgressExamples sets the shared state `completed` to true it will
    // trigger Dioxus to re-render this (i.e. `App`) component.
    let signal1 = use_atom_ref(&cx, |_| false);
    let signal2 = use_atom_ref(&cx, |_| false);
    let signal3 = use_atom_ref(&cx, |_| false);
    let signal4 = use_atom_ref(&cx, |_| false);

    cx.render(rsx! {
        style { "{get_bulma_css()}" },
        ProgressExample1 { id: "1" completed: signal1 }
        ProgressExample2 { id: "2" completed: signal2 }
        ProgressExample3 { id: "3", completed: signal3 }
        ProgressExample4 { id: "4", completed: signal4 }
    })
}

#[allow(non_snake_case)]
#[derive(Props)]
pub struct ProgressExampleProps<'a> {
    #[props(!optional)]
    id: &'a str,

    #[props(!optional)]
    completed: &'a UseAtomRef<bool>,
}

#[allow(non_snake_case)]
fn ProgressExample1<'a>(cx: Scope<'a, ProgressExampleProps<'a>>) -> Element {
    let state = use_atom_ref(&cx, PROGRESS1);
    let value = state.read().value();
    log::trace!("ProgressExample[{}]: render, value: {}", cx.props.id, value);

    // Reset progress on completion
    if state.read().completed() {
        state.write().reset();
    }

    cx.render(rsx! {
        Section { class: "py-2".into(),
            SubTitle { "Restarting progress" }
            Progress { id: cx.props.id.into(),
                state: state,
                color: Colors::Primary,
                completed: cx.props.completed,
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
    log::trace!("ProgressExample[{}]: render, value: {}", cx.props.id, value);

    cx.render(rsx! {
        Section { class: "py-2".into(),
            SubTitle { "Regular progress" }
            Progress { id: cx.props.id.into(),
                state: state,
                color: Colors::Info,
                completed: cx.props.completed,
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
            SubTitle { "Timed 1 sec progress restarting" }
            ProgressTimed { id: id.clone(),
                state: state,
                duration: 1000,
                color: Colors::Danger,
                completed: cx.props.completed,
            }
            Button {
                class: "ml-5".into(),
                color: Colors::Warning,
                onclick: move |_| {
                    // state.write().reset();
                },
                "Reset progress {id.clone()}"
            }
            Button {
                class: "ml-5".into(),
                color: Colors::Success,
                onclick: move |_| {
                    // state.write().complete();
                },
                "Complete progress {id.clone()}"
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
    let id = format!("{}", *id4.read());

    cx.render(rsx! {
        Section { class: "py-2".into(),
            SubTitle { "Timed 5 sec progress restartable" }
            ProgressTimed { id: id.clone(),
                state: state,
                duration: 5000,
                color: Colors::Warning,
                completed: cx.props.completed,
            }
            Button {
                class: "ml-5".into(),
                color: Colors::Warning,
                onclick: move |_| {
                    state.write().reset();
                    *id4.write_silent() += 1;
                },
                "Reset progress {id.clone()}"
            }
            Button {
                class: "ml-5".into(),
                color: Colors::Success,
                onclick: move |_| {
                    state.write().complete();
                },
                "Complete progress {id.clone()}"
            }
        }
    })
}
