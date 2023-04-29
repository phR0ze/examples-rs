//! Dioxus Bulma example
//!

use bulma::{
    elements::Button,
    layouts::{Column, Columns, Container},
    prelude::*,
};

fn main() {
    #[cfg(target_family = "wasm")]
    dioxus_web::launch(App);

    #[cfg(any(windows, unix))]
    dioxus_desktop::launch_cfg(
        App,
        dioxus_desktop::Config::new().with_window(
            dioxus_desktop::WindowBuilder::new()
                .with_title("Bulma Example")
                .with_resizable(true)
                //.with_transparent(true)
                //.with_decorations(false)
                .with_inner_size(dioxus_desktop::LogicalSize::new(1200, 700)),
        ),
    )
}

// UI entry point
#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        style { "{get_bulma_css()}" },
        Container {
            is_fluid: true,
            br {}
            Columns {
                Column {
                    Button {
                        color: Colors::Success,
                        is_fullwidth: true,
                        onclick: move |_| {
                            println!("success");
                        }
                        "Success"
                    }
                }
                Column {
                    Button {
                        color: Colors::Info,
                        is_fullwidth: true,
                        onclick: move |_| {
                            println!("info");
                        }
                        "Info"
                    }
                }
                Column {
                    Button {
                        color: Colors::Warning,
                        is_fullwidth: true,
                        onclick: move |_| {
                            println!("warning");
                        }
                        "Warning"
                    }
                }
                Column {
                    Button {
                        color: Colors::Danger,
                        is_fullwidth: true,
                        onclick: move |_| {
                            println!("danger");
                        }
                        "Danger"
                    }
                }
            }
            Columns {
                Column {
                    Button {
                        color: Colors::Success,
                        is_fullwidth: true,
                        onclick: move |_| {
                            println!("success");
                        }
                        "Success"
                    }
                }
                Column {
                    Button {
                        color: Colors::Info,
                        is_fullwidth: true,
                        onclick: move |_| {
                            println!("info");
                        }
                        "Info"
                    }
                }
                Column {
                    Button {
                        color: Colors::Warning,
                        is_fullwidth: true,
                        onclick: move |_| {
                            println!("warning");
                        }
                        "Warning"
                    }
                }
                Column {
                    Button {
                        color: Colors::Danger,
                        is_fullwidth: true,
                        onclick: move |_| {
                            println!("danger");
                        }
                        "Danger"
                    }
                }
            }
        }
    })
}
