use bulma::{
    elements::{Button, Progress, ProgressTimed},
    layouts::{Column, Columns, Section},
    prelude::*,
};

static PROGRESS_STATE: fermi::AtomRef<ProgressState> = |_| ProgressState::default();

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
    println!("render app");
    fermi::use_init_atom_root(&cx);

    cx.render(rsx! {
        style { "{get_bulma_css()}" },
        ProgressExamples {}
    })
}

// By splitting out the progress examples into a separate component we can track
// rendering calls separately from the the parent component
#[allow(non_snake_case)]
fn ProgressExamples(cx: Scope) -> Element {
    println!("render progress examples");
    let state = fermi::use_atom_ref(&cx, PROGRESS_STATE);

    let progress1 = "progress1";
    let progress2 = "progress2";
    let progress3 = "progress3";
    let progress4 = "progress4";
    let value1 = state.read().value(progress1);
    let value2 = state.read().value(progress2);
    let value3 = state.read().value(progress3);

    cx.render(rsx! {
        Section {
            Columns {
                Column {
                    Progress { id: progress1,
                        state: state,
                        color: Colors::Primary,
                    }
                    Button {
                        color: Colors::Primary,
                        onclick: move |_| {
                            state.write().set(progress1, value1 + 0.05)
                        },
                        "Increment progress 1"
                    }
                    Button {
                        class: "ml-5".into(),
                        color: Colors::Warning,
                        onclick: move |_| {
                            state.write().reset(progress1);
                        },
                        "Reset progress 1"
                    }
                    Button {
                        class: "ml-5".into(),
                        color: Colors::Success,
                        onclick: move |_| {
                            state.write().complete(progress1);
                        },
                        "Complete progress 1"
                    }
                }
            }

            Columns {
                Column {
                    Progress { id: progress2,
                        state: state,
                        color: Colors::Info,
                    }
                    Button {
                        color: Colors::Info,
                        onclick: move |_| {
                            state.write().set(progress2, value2 + 0.05)
                        },
                        "Increment progress 2"
                    }
                    Button {
                        class: "ml-5".into(),
                        color: Colors::Warning,
                        onclick: move |_| {
                            state.write().reset(progress2);
                        },
                        "Reset progress 2"
                    }
                    Button {
                        class: "ml-5".into(),
                        color: Colors::Success,
                        onclick: move |_| {
                            state.write().complete(progress2);
                        },
                        "Complete progress 2"
                    }
                }
            }

            Columns {
                Column {
                    Progress { id: progress3,
                        state: state,
                        color: Colors::Danger,
                        value: 0.5,
                    }
                    Button {
                        color: Colors::Info,
                        is_light: true,
                        onclick: move |_| {
                            state.write().set(progress3, value3 + 0.05)
                        },
                        "Increment progress 3"
                    }
                    Button {
                        class: "ml-5".into(),
                        color: Colors::Warning,
                        onclick: move |_| {
                            state.write().reset(progress3);
                        },
                        "Reset progress 3"
                    }
                    Button {
                        class: "ml-5".into(),
                        color: Colors::Success,
                        onclick: move |_| {
                            state.write().complete(progress3);
                        },
                        "Complete progress 3"
                    }
                }
            }

            Columns {
                Column {
                    ProgressTimed { id: progress4,
                        state: state,
                        color: Colors::Warning,
                    }
                    Button {
                        class: "ml-5".into(),
                        color: Colors::Warning,
                        onclick: move |_| {
                            state.write().reset(progress4);
                        },
                        "Reset progress 4"
                    }
                    Button {
                        class: "ml-5".into(),
                        color: Colors::Success,
                        onclick: move |_| {
                            state.write().complete(progress4);
                        },
                        "Complete progress 4"
                    }
                }
            }
        }
    })
}
