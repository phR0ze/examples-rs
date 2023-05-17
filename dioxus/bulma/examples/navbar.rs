use bulma::{components::*, elements::*, layouts::*, prelude::*};

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
                    NavbarItem {
                        onclick: move |_| use_router(cx).push_route("/3", None, None),
                        "Page 3"
                    }
                    NavbarDropdown {
                        title: "Pages".into(),
                        NavbarItem {
                            onclick: move |_| use_router(cx).push_route("/", None, None),
                            "Page 1"
                        }
                        NavbarItem {
                            onclick: move |_| use_router(cx).push_route("/2", None, None),
                            "Page 2"
                        }
                        NavbarItem {
                            onclick: move |_| use_router(cx).push_route("/3", None, None),
                            "Page 3"
                        }
                    }
                }
            }
        }
    })
}

#[allow(non_snake_case)]
fn Page1(cx: Scope) -> Element {
    cx.render(rsx! { Section { Title { "Page 1"} } })
}

#[allow(non_snake_case)]
fn Page2(cx: Scope) -> Element {
    cx.render(rsx! { Section { Title { "Page 2"} } })
}

#[allow(non_snake_case)]
fn Page3(cx: Scope) -> Element {
    cx.render(rsx! { Section { Title { "Page 3"} } })
}
