use bulma::{
    elements::{Button, Progress, ProgressTimed},
    layouts::{Column, Columns, Section},
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

    let progress1 = "progress1";
    let progress2 = "progress2";
    let progress3 = "progress3";
    let progress4 = "progress4";
    let value1 = state.read().progress.value(progress1);
    let value2 = state.read().progress.value(progress2);
    let value3 = state.read().progress.value(progress3);

    cx.render(rsx! {
        style { "{get_bulma_css()}" },
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
                            state.write().progress.set(progress1, value1 + 0.05)
                        },
                        "Increment progress 1"
                    }
                    Button {
                        class: "ml-5".into(),
                        color: Colors::Warning,
                        onclick: move |_| {
                            state.write().progress.reset(progress1);
                        },
                        "Reset progress 1"
                    }
                    Button {
                        class: "ml-5".into(),
                        color: Colors::Success,
                        onclick: move |_| {
                            state.write().progress.complete(progress1);
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
                            state.write().progress.set(progress2, value2 + 0.05)
                        },
                        "Increment progress 2"
                    }
                    Button {
                        class: "ml-5".into(),
                        color: Colors::Warning,
                        onclick: move |_| {
                            state.write().progress.reset(progress2);
                        },
                        "Reset progress 2"
                    }
                    Button {
                        class: "ml-5".into(),
                        color: Colors::Success,
                        onclick: move |_| {
                            state.write().progress.complete(progress2);
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
                            state.write().progress.set(progress3, value3 + 0.05)
                        },
                        "Increment progress 3"
                    }
                    Button {
                        class: "ml-5".into(),
                        color: Colors::Warning,
                        onclick: move |_| {
                            state.write().progress.reset(progress3);
                        },
                        "Reset progress 3"
                    }
                    Button {
                        class: "ml-5".into(),
                        color: Colors::Success,
                        onclick: move |_| {
                            state.write().progress.complete(progress3);
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
                        color: Colors::Danger,
                        is_light: true,
                        onclick: move |_| {
                            //state.write().progress.set(progress4, value4 + 0.05)
                        },
                        "Pause progress 4"
                    }
                    Button {
                        class: "ml-5".into(),
                        color: Colors::Warning,
                        onclick: move |_| {
                            state.write().progress.reset(progress4);
                        },
                        "Reset progress 4"
                    }
                    Button {
                        class: "ml-5".into(),
                        color: Colors::Success,
                        onclick: move |_| {
                            state.write().progress.complete(progress4);
                        },
                        "Complete progress 4"
                    }
                }
            }
        }
    })
}
