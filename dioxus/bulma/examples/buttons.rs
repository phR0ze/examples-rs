use bulma::{elements::*, layouts::*, prelude::*};

fn main() {
    #[cfg(target_family = "wasm")]
    dioxus_web::launch(App);

    #[cfg(any(windows, unix))]
    dioxus_desktop::launch_cfg(
        App,
        dioxus_desktop::Config::new().with_window(
            dioxus_desktop::WindowBuilder::new()
                .with_resizable(true)
                .with_inner_size(dioxus_desktop::LogicalSize::new(1200, 700)),
        ),
    )
}

// UI entry point
#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        style { "{get_bulma_css()}" },
        Section {
            Title { "Colors" }
            Columns {
                Column {
                    Button {
                        color: Colors::White,
                        "White"
                    }
                }
                Column {
                    Button {
                        color: Colors::Light,
                        "Light"
                    }
                }
                Column {
                    Button {
                        color: Colors::Dark,
                        "Dark"
                    }
                }
                Column {
                    Button {
                        color: Colors::Black,
                        "Black"
                    }
                }
                Column {
                    Button {
                        color: Colors::Text,
                        "Text"
                    }
                }
                Column {
                    Button {
                        color: Colors::Ghost,
                        "Ghost"
                    }
                }
            }
            Columns {
                Column {
                    Button {
                        color: Colors::Primary,
                        "Primary"
                    }
                }
                Column {
                    Button {
                        color: Colors::Link,
                        "Link"
                    }
                }
                Column {
                    Button {
                        color: Colors::Info,
                        "Info"
                    }
                }
                Column {
                    Button {
                        color: Colors::Success,
                        "Success"
                    }
                }
                Column {
                    Button {
                        color: Colors::Warning,
                        "Warning"
                    }
                }
                Column {
                    Button {
                        color: Colors::Danger,
                        "Danger"
                    }
                }
            }
        }
        Section {
            Title { "Light Colors" }
            Columns {
                Column {
                    Button {
                        color: Colors::White,
                        is_light: true,
                        "White"
                    }
                }
                Column {
                    Button {
                        color: Colors::Light,
                        is_light: true,
                        "Light"
                    }
                }
                Column {
                    Button {
                        color: Colors::Dark,
                        is_light: true,
                        "Dark"
                    }
                }
                Column {
                    Button {
                        color: Colors::Black,
                        is_light: true,
                        "Black"
                    }
                }
                Column {
                    Button {
                        color: Colors::Text,
                        is_light: true,
                        "Text"
                    }
                }
                Column {
                    Button {
                        color: Colors::Ghost,
                        is_light: true,
                        "Ghost"
                    }
                }
            }
            Columns {
                Column {
                    Button {
                        color: Colors::Primary,
                        is_light: true,
                        "Primary"
                    }
                }
                Column {
                    Button {
                        color: Colors::Link,
                        is_light: true,
                        "Link"
                    }
                }
                Column {
                    Button {
                        color: Colors::Info,
                        is_light: true,
                        "Info"
                    }
                }
                Column {
                    Button {
                        color: Colors::Success,
                        is_light: true,
                        "Success"
                    }
                }
                Column {
                    Button {
                        color: Colors::Warning,
                        is_light: true,
                        "Warning"
                    }
                }
                Column {
                    Button {
                        color: Colors::Danger,
                        is_light: true,
                        "Danger"
                    }
                }
            }
        }
    })
}
