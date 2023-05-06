//! Dioxus Bulma example
//!

use bulma::{
    components::{Card, CardContent, CardHeader, CardImage},
    elements::{Image, SubTitle, Tag, TagLink, Tags, Title},
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
                    size: 4,
                    Tags {
                        Tag {
                            color: Colors::Warning,
                            size: Sizes::Medium,
                            "Rust"
                        }
                        Tag {
                            color: Colors::Link,
                            size: Sizes::Medium,
                            "Go"
                        }
                        Tag {
                            color: Colors::Info,
                            size: Sizes::Medium,
                            "Python"
                        }
                        Tag {
                            color: Colors::Danger,
                            size: Sizes::Medium,
                            "Ruby"
                        }
                        Tag {
                            color: Colors::Dark,
                            size: Sizes::Medium,
                            "C++"
                        }
                    }
                }
                Column {
                    size: 3,
                    Tags {
                        Tag {
                            color: Colors::Danger,
                            size: Sizes::Medium,
                            deletable: true,
                            "React"
                        }
                        Tag {
                            color: Colors::Success,
                            size: Sizes::Medium,
                            deletable: true,
                            "Vue"
                        }
                        Tag {
                            color: Colors::Dark,
                            size: Sizes::Medium,
                            deletable: true,
                            "Dioxus"
                        }
                    }
                }
                Column {
                    size: 4,
                    div {
                        class: "field is-grouped is-grouped-multiline",
                        div {
                            class: "control",
                            Tags {
                                addons: true,
                                Tag {
                                    color: Colors::Dark,
                                    size: Sizes::Medium,
                                    "crates.io"
                                }
                                Tag {
                                    color: Colors::Warning,
                                    size: Sizes::Medium,
                                    "v0.2.4"
                                }
                            }
                        }
                        div {
                            class: "control",
                            Tags {
                                addons: true,
                                Tag {
                                    color: Colors::Dark,
                                    size: Sizes::Medium,
                                    "docs"
                                }
                                Tag {
                                    color: Colors::Info,
                                    size: Sizes::Medium,
                                    "latest"
                                }
                            }
                        }
                    }
                }
                Column {
                    size: 1,
                    TagLink {
                        color: Colors::Link,
                        size: Sizes::Medium,
                        onclick: |_| {
                            //toast.write().popup(ToastInfo::simple("clickable tag clicked."));
                        }
                        "Link"
                    }
                }
            }
        }
    })
}
