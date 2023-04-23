use common::icons::outline::Shape as Icon;
use common::STATIC_ARGS;
use dioxus::prelude::*;
use dioxus_router::{use_router, Route, Router};
use kit::{
    components::{
        nav::{Nav, Route as UIRoute},
        section::Section,
        titlebar::Titlebar,
    },
    elements::{
        button::Button,
        checkbox::Checkbox,
        switch::Switch,
        tooltip::{ArrowPosition, Tooltip},
        Appearance,
    },
    layout::{sidebar::Sidebar as ReusableSidebar, topbar::Topbar},
    STYLE,
};
use ui::{utils::get_available_themes, APP_STYLE};

// Application state
#[derive(Default)]
struct State {
    splash_viewed: bool,
    sidebar_hidden: bool,
}

pub struct AppRoutes<'a> {
    pub loading: &'a str,
    pub chat: &'a str,
    pub friends: &'a str,
    pub files: &'a str,
    pub settings: &'a str,
}

#[derive(PartialEq, Clone)]
pub struct RouteInfo {
    pub routes: Vec<UIRoute>,
    pub active: UIRoute,
}

pub static APP_ROUTES: AppRoutes =
    AppRoutes { loading: "/", chat: "/chat", friends: "/friends", files: "/files", settings: "/settings" };

fn main() {
    dioxus_desktop::launch_cfg(
        App,
        dioxus_desktop::Config::new().with_window(
            dioxus_desktop::WindowBuilder::new()
                .with_title("crux")
                .with_resizable(true)
                // Provides rounded window corner effect
                .with_transparent(true)
                // Turns off standard window manager controls
                .with_decorations(false)
                // We start the min inner size smaller because the prelude pages like unlock can be rendered much smaller.
                .with_min_inner_size(dioxus_desktop::LogicalSize::new(300.0, 350.0))
                .with_inner_size(dioxus_desktop::LogicalSize::new(950.0, 600.0)),
        ),
    )
}

// UI entry point
#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    println!("Rendering App layout");
    use_shared_state_provider(cx, || State::default());
    let theme = get_available_themes().iter().find(|x| x.name == "Nord").unwrap().styles.clone();

    cx.render(rsx! {
        style { "{STYLE} {APP_STYLE} {theme}" },
        div {
            Titlebar {
                text: "Pre-release | Issues/Feedback".into(),
                link: "https://issues.satellite.im".into()
            },
            Routes{},
        }
    })
}

#[derive(PartialEq, Props)]
pub struct Props {
    route_info: RouteInfo,
}

#[allow(non_snake_case)]
fn ChatLayout(cx: Scope<Props>) -> Element {
    let state = use_shared_state::<State>(cx)?;

    let cta_text = "Things are better with friends";
    let image_path = STATIC_ARGS
        .extras_path
        .join("images")
        .join("mascot")
        .join("better_with_friends.webp")
        .to_str()
        .map(|x| x.to_string())
        .unwrap_or_default();

    cx.render(rsx! {
        div {
            id: "chat-layout",
            aria_label: "chat-layout",
            Sidebar {
                route_info: cx.props.route_info.clone()
            },
            div {
                id: "welcome",
                aria_label: "welcome-screen",
                if state.read().sidebar_hidden {
                    rsx!(
                        Topbar {
                            with_back_button: state.read().sidebar_hidden,
                            onback: move |_| {
                                state.write().sidebar_hidden = true;
                            },
                        },
                    )
                }
                img {
                    class: "image",
                    src:"{image_path}"
                },
                p {
                    class: "muted",
                    "{cta_text}"
                },
                Button {
                    icon: Icon::Plus,
                    aria_label: "add-friends-button".into(),
                    text: "Add Someone".into(),
                    appearance: Appearance::Secondary,
                    onpress: move |_| {
                        use_router(cx).replace_route(APP_ROUTES.friends, None, None);
                    }
                },
            }
        }
    })
}

#[allow(non_snake_case)]
fn Settings(cx: Scope<Props>) -> Element {
    cx.render(rsx! {
        div {
            id: "settings-layout",
            aria_label: "settings-layout",
            Sidebar {
                route_info: cx.props.route_info.clone(),
            },
            div {
                class: "full-width flex",
                div {
                    id: "content",
                    class: "full-width",
                    p {
                        "hello world",
                    }
                },
            },
        }
    })
}

#[allow(non_snake_case)]
fn Sidebar(cx: Scope<Props>) -> Element {
    cx.render(rsx! {
        ReusableSidebar {
            with_nav: cx.render(rsx!(
                Nav {
                    routes: cx.props.route_info.routes.clone(),
                    active: cx.props.route_info.active.clone(),
                    onnavigate: move |route| {
                        //use_router(cx).replace_route(route, None, None);
                    }
                },
            )),
        }
    })
}

#[allow(non_snake_case)]
fn Routes(cx: Scope) -> Element {
    let chat_route = UIRoute {
        to: APP_ROUTES.chat,
        name: "Chat".into(),
        icon: Icon::ChatBubbleBottomCenterText,
        ..UIRoute::default()
    };
    let settings_route =
        UIRoute { to: APP_ROUTES.settings, name: "Settings".into(), icon: Icon::Cog6Tooth, ..UIRoute::default() };
    let friends_route =
        UIRoute { to: APP_ROUTES.friends, name: "Friends".into(), icon: Icon::Users, ..UIRoute::default() };
    let files_route =
        UIRoute { to: APP_ROUTES.files, name: "Files".into(), icon: Icon::Folder, ..UIRoute::default() };
    let routes = vec![
        chat_route.clone(),
        files_route.clone(),
        friends_route.clone(),
        settings_route.clone(),
    ];

    cx.render(rsx! {
        Router {
            Route {
                to: APP_ROUTES.loading,
                Splash{},
            },
            Route {
                to: APP_ROUTES.chat,
                ChatLayout {
                    route_info: RouteInfo {
                        routes: routes.clone(),
                        active: chat_route.clone(),
                    }
                }
            },
            Route {
                to: APP_ROUTES.settings,
                Settings {
                    route_info: RouteInfo {
                        routes: routes.clone(),
                        active: settings_route.clone(),
                    }
                }
            },
            Route {
                to: APP_ROUTES.friends,
                Settings {
                    route_info: RouteInfo {
                        routes: routes.clone(),
                        active: settings_route.clone(),
                    }
                }
            },
            Route {
                to: APP_ROUTES.files,
                Settings {
                    route_info: RouteInfo {
                        routes: routes.clone(),
                        active: settings_route.clone(),
                    }
                }
            }
        }
    })
}

// Splash screen example
#[allow(non_snake_case)]
pub fn Splash(cx: Scope) -> Element {
    println!("Rendering Splash layout");
    let state = use_shared_state::<State>(cx)?;
    let img_path =
        STATIC_ARGS.extras_path.join("assets").join("img").join("uplink.gif").to_string_lossy().to_string();
    if !state.read().splash_viewed {
        cx.render(rsx! {
            div {
                onclick: move |_| {
                    state.write().splash_viewed = true;
                },
                img { style: "width: 100%", src: "{img_path}" }
            }
        })
    } else {
        use_router(cx).replace_route(APP_ROUTES.chat, None, None);
        None
    }
}

// Blob of content for testing
#[allow(non_snake_case)]
fn Content(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            Section {
                section_label: "Checkbox - unchecked".into(),
                section_description: "Example of a checkbox that is unchecked".into(),
                Checkbox{
                    disabled: false,
                    width: "1.5em".into(),
                    height: "1.5em".into(),
                    is_checked: false,
                    on_click: move |_| { },
                },
            },
            Section {
                section_label: "Checkbox - checked".into(),
                section_description: "Example of a checkbox that is checked".into(),
                Checkbox{
                    disabled: false,
                    width: "1.5em".into(),
                    height: "1.5em".into(),
                    is_checked: true,
                    on_click: move |_| { },
                },
            },
            Section {
                section_label: "Inactive switch".into(),
                section_description: "Example of a switch that is inactive by default".into(),
                Switch {
                    active: false,
                },
            },
            Section {
                section_label: "Active switch".into(),
                section_description: "Example of a switch that is active by default".into(),
                Switch {
                    active: true,
                }
            },
            div {
                style: "border-bottom: 1px solid var(--border-color)",
                p {
                    "Button Examples",
                },
                Button {
                    text: "Default".into(),
                    appearance: kit::elements::Appearance::Default,
                    icon: Icon::Check,
                },
                Button {
                    text: "Primary".into(),
                    appearance: kit::elements::Appearance::Primary,
                    icon: Icon::Check,
                },
                Button {
                    text: "Secondary".into(),
                    appearance: kit::elements::Appearance::Secondary,
                    icon: Icon::Check,
                },
                Button {
                    text: "SecondaryLess".into(),
                    appearance: kit::elements::Appearance::SecondaryLess,
                    icon: Icon::Check,
                },
                Button {
                    text: "Success".into(),
                    appearance: kit::elements::Appearance::Success,
                    icon: Icon::Check,
                },
                Button {
                    text: "Danger".into(),
                    appearance: kit::elements::Appearance::Danger,
                    icon: Icon::Check,
                },
                Button {
                    text: "Disabled".into(),
                    appearance: kit::elements::Appearance::Disabled,
                    icon: Icon::Check,
                },
                Button {
                    text: "Transparent".into(),
                    appearance: kit::elements::Appearance::Transparent,
                    icon: Icon::Check,
                }
                Button {
                    text: "With Badge".into(),
                    with_badge: "Badge".into(),
                    appearance: kit::elements::Appearance::Default,
                    icon: Icon::Check,
                },
            },
            Section {
                section_label: "Small Button".into(),
                section_description: "Setting small=true provides room only for the icon".into(),
                Button {
                    small: true,
                    icon: Icon::Check,
                },
            },
            Section {
                section_label: "Custom Tooltip".into(),
                section_description: "Creating a custom tool tip for the button".into(),
                Button {
                    text: "Custom Tooltip".into(),
                    icon: Icon::Cog6Tooth,
                    appearance: kit::elements::Appearance::Primary,
                    tooltip: cx.render(rsx!(
                        Tooltip {
                            arrow_position: ArrowPosition::Bottom,
                            text: String::from("Settings")
                        }
                    )),
                },
            }
        }
    })
}
