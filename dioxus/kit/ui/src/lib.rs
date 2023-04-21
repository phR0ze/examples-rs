use common::icons::outline::Shape as Icon;
use common::icons::Icon as IconElement;
use common::language::{change_language, get_local_text};
use dioxus::prelude::*;
use dioxus_desktop::{LogicalSize, WindowBuilder};
use once_cell::sync::Lazy;
use std::sync::Arc;
use warp::multipass;
use window_manager::WindowManagerCmdChannels;

pub mod components;
pub mod layouts;
pub mod logger;
pub mod overlay;
pub mod utils;
pub mod window_manager;

pub const APP_STYLE: &str = include_str!("./compiled_styles.css");

pub static OPEN_DYSLEXIC: &str = include_str!("./open-dyslexic.css");

pub struct UplinkRoutes<'a> {
    pub loading: &'a str,
    pub chat: &'a str,
    pub friends: &'a str,
    pub files: &'a str,
    pub settings: &'a str,
}

// used to close the popout player, among other things
pub static WINDOW_CMD_CH: Lazy<WindowManagerCmdChannels> = Lazy::new(|| {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    WindowManagerCmdChannels { tx, rx: Arc::new(tokio::sync::Mutex::new(rx)) }
});

pub static UPLINK_ROUTES: UplinkRoutes =
    UplinkRoutes { loading: "/", chat: "/chat", friends: "/friends", files: "/files", settings: "/settings" };

// serve as a sort of router while the user logs in]
#[allow(clippy::large_enum_variant)]
#[derive(PartialEq, Eq)]
pub enum AuthPages {
    Unlock,
    CreateAccount,
    Success(multipass::identity::Identity),
}

pub fn get_window_builder(with_predefined_size: bool) -> WindowBuilder {
    let title = get_local_text("uplink");

    #[allow(unused_mut)]
    let mut window = WindowBuilder::new()
        .with_title(title)
        .with_resizable(true)
        // We start the min inner size smaller because the prelude pages like unlock can be rendered much smaller.
        .with_min_inner_size(LogicalSize::new(300.0, 350.0));

    if with_predefined_size {
        window = window.with_inner_size(LogicalSize::new(950.0, 600.0));
    }
    window = window.with_decorations(false).with_transparent(true);
    window
}

pub fn get_pre_release_message(_cx: Scope) -> Element {
    let pre_release_text = get_local_text("uplink.pre-release");
    _cx.render(rsx!(
        div {
            id: "pre-release",
            aria_label: "pre-release",
            IconElement {
                icon: Icon::Beaker,
            },
            p {
                div {
                    onclick: move |_| {
                        let _ = open::that("https://issues.satellite.im");
                    },
                    "{pre_release_text}"
                }

            }
        },
    ))
}
