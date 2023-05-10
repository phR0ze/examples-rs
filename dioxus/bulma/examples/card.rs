use bulma::{
    components::{Card, CardContent, CardFooter, CardHeader, CardImage},
    elements::{Image, SubTitle, Title},
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
                    Card {
                        CardHeader { title: "Header Title First".into() }
                        CardContent {
                            Title { "Content before image" }
                            SubTitle { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus nec iaculis mauris." }
                        }
                        CardImage {
                            Image {
                                src: "https://bulma.io/images/placeholders/1280x960.png",
                                ratio: (16, 9).into(),
                            }
                        }
                    }
                }
                Column {
                    Card {
                        CardContent {
                            Title { "Hello World" }
                            SubTitle { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus nec iaculis mauris." }
                        }
                        CardImage {
                            Image {
                                src: "https://bulma.io/images/placeholders/1280x960.png",
                                ratio: (16, 9).into(),
                            }
                        }
                        CardHeader { title: "Header Title Last".into() }
                    }
                }
               Column {
                    Card {
                        CardImage {
                            Image {
                                src: "https://bulma.io/images/placeholders/1280x960.png",
                                ratio: (16, 9).into(),
                            }
                        }
                        CardContent {
                            Title { "Hello World" }
                            SubTitle { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus nec iaculis mauris." }
                        }
                        CardFooter {
                            Title { "Card Footer" }
                        }
                    }
                }
                Column {
                    Card {
                        CardImage {
                            Image {
                                src: "https://bulma.io/images/placeholders/1280x960.png",
                                ratio: (16, 9).into(),
                            }
                        }
                        CardContent {
                            Title { "Hello World" }
                            SubTitle { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus nec iaculis mauris." }
                        }
                    }
                }
            }
        }
    })
}
