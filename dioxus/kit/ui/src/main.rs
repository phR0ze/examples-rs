#![windows_subsystem = "windows"]
// the above macro will make uplink be a "window" application instead of a  "console" application for Windows.

use chrono::{Datelike, Local, Timelike};
use clap::Parser;
use common::icons::outline::Shape as Icon;
use common::icons::Icon as IconElement;
use common::language::{change_language, get_local_text};
use common::{state, warp_runner, LogProfile, STATIC_ARGS, WARP_CMD_CH, WARP_EVENT_CH};
use dioxus::prelude::*;
use dioxus_desktop::tao::dpi::LogicalSize;
use dioxus_desktop::tao::event::WindowEvent;
use dioxus_desktop::tao::menu::AboutMetadata;
use dioxus_desktop::Config;
use dioxus_desktop::{tao, use_window};
use futures::channel::oneshot;
use futures::StreamExt;
use kit::components::context_menu::{ContextItem, ContextMenu};
use kit::components::nav::Route as UIRoute;
use kit::elements::button::Button;
use kit::elements::Appearance;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use once_cell::sync::Lazy;
use rfd::FileDialog;
use std::collections::{HashMap, HashSet};
use ui::overlay::{make_config, OverlayDom};

use std::process::Command;
use std::time::Instant;
use std::{fs, io};
use uuid::Uuid;
use warp::multipass;
use warp::multipass::identity::Platform;

use std::sync::Arc;
use tao::menu::{MenuBar as Menu, MenuItem};
use tao::window::WindowBuilder;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{sleep, Duration};
use warp::logging::tracing::log::{self, LevelFilter};

use dioxus_desktop::use_wry_event_handler;
use dioxus_desktop::wry::application::event::Event as WryEvent;

use ui::components::debug_logger::DebugLogger;
use ui::components::toast::Toast;
use ui::layouts::create_account::CreateAccountLayout;
use ui::layouts::friends::FriendsLayout;
use ui::layouts::loading::LoadingLayout;
use ui::layouts::settings::SettingsLayout;
use ui::layouts::storage::{FilesLayout, DRAG_EVENT};
use ui::layouts::unlock::UnlockLayout;

use common::{
    state::{friends, storage, ui::WindowMeta, Action, State},
    warp_runner::{ConstellationCmd, MultiPassCmd, RayGunCmd, WarpCmd},
};
use dioxus_router::*;
use std::panic;
use ui::utils::auto_updater::{
    get_download_dest, DownloadProgress, DownloadState, SoftwareDownloadCmd, SoftwareUpdateCmd,
};
use ui::utils::get_available_themes;
use ui::window_manager::WindowManagerCmdChannels;
use ui::{components::chat::RouteInfo, layouts::chat::ChatLayout};

use kit::STYLE as UIKIT_STYLES;
use ui::APP_STYLE;

fn main() {
    std::fs::create_dir_all(&STATIC_ARGS.uplink_path).expect("Error creating Uplink directory");
    std::fs::create_dir_all(&STATIC_ARGS.warp_path).expect("Error creating Warp directory");
    std::fs::create_dir_all(&STATIC_ARGS.themes_path).expect("error creating themes directory");
    std::fs::create_dir_all(&STATIC_ARGS.fonts_path).expect("error fonts themes directory");

    dioxus_desktop::launch_cfg(
        bootstrap,
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

// start warp_runner and ensure the user is logged in
fn bootstrap(cx: Scope) -> Element {
    // warp_runner must be started from within a tokio reactor
    // store in a use_ref to make it not get dropped
    let warp_runner = use_ref(cx, warp_runner::WarpRunner::new);
    warp_runner.write_silent().run();

    // make the window smaller while the user authenticates
    let desktop = use_window(cx);
    desktop.set_inner_size(LogicalSize { width: 500.0, height: 350.0 });

    cx.render(rsx!(crate::auth_page_manager {}))
}

// Uplink's Router depends on State, which can't be loaded until the user logs in.
// don't see a way to replace the router
// so instead use a Prop to determine which page to render
// after the user logs in, app_bootstrap loads Uplink as normal.
fn auth_page_manager(cx: Scope) -> Element {
    let page = use_state(cx, || ui::AuthPages::Unlock);
    let pin = use_ref(cx, String::new);
    cx.render(rsx!(match &*page.current() {
        ui::AuthPages::Success(ident) => rsx!(app_bootstrap { identity: ident.clone() }),
        _ => rsx!(auth_wrapper { page: page.clone(), pin: pin.clone() }),
    }))
}

#[inline_props]
fn auth_wrapper(cx: Scope, page: UseState<ui::AuthPages>, pin: UseRef<String>) -> Element {
    cx.render(rsx! (
        style { "{UIKIT_STYLES} {APP_STYLE}" },
        div {
            id: "app-wrap",
            TitleBar{},
            match *page.current() {
                ui::AuthPages::Unlock => rsx!(UnlockLayout { page: page.clone(), pin: pin.clone() }),
                ui::AuthPages::CreateAccount => rsx!(CreateAccountLayout { page: page.clone(), pin: pin.clone() }),
                _ => panic!("invalid page")
            }
        }
    ))
}

#[allow(non_snake_case)]
fn TitleBar(cx: Scope) -> Element {
    let desktop = use_window(cx);
    cx.render(rsx!(
        div {
            id: "titlebar",
            onmousedown: move |_| { desktop.drag(); },
            div {
                class: "controls",
                Button {
                    aria_label: "minimize-button".into(),
                    icon: Icon::Minus,
                    appearance: Appearance::Transparent,
                    onpress: move |_| {
                        desktop.set_minimized(true);
                    }
                },
                Button {
                    aria_label: "square-button".into(),
                    icon: Icon::Square2Stack,
                    appearance: Appearance::Transparent,
                    onpress: move |_| {
                        desktop.set_maximized(!desktop.is_maximized());
                    }
                },
                Button {
                    aria_label: "close-button".into(),
                    icon: Icon::XMark,
                    appearance: Appearance::Transparent,
                    onpress: move |_| {
                        desktop.close();
                    }
                },
            },
        }
    ))
}

// called at the end of the auth flow
#[inline_props]
pub fn app_bootstrap(cx: Scope, identity: multipass::identity::Identity) -> Element {
    log::trace!("rendering app_bootstrap");
    let mut state = State::load();

    if STATIC_ARGS.use_mock {
        assert!(state.friends().initialized);
        assert!(state.chats().initialized);
    } else {
        state.set_own_identity(identity.clone().into());
    }

    // Reload theme from file if present
    let themes = get_available_themes();
    let theme = themes.iter().find(|t| state.ui.theme.as_ref().map(|theme| theme.eq(t)).unwrap_or_default());
    if let Some(t) = theme {
        state.set_theme(Some(t.clone()));
    }

    // set the window to the normal size.
    // todo: perhaps when the user resizes the window, store that in State, and load that here
    let desktop = use_window(cx);
    // Here we set the size larger, and bump up the min size in preparation for rendering the main app.
    desktop.set_inner_size(LogicalSize::new(950.0, 600.0));
    desktop.set_min_inner_size(Some(LogicalSize::new(300.0, 500.0)));

    // todo: delete this. it is just an example
    if state.configuration.general.enable_overlay {
        let overlay_test = VirtualDom::new(OverlayDom);
        let window = desktop.new_window(overlay_test, make_config());
        state.ui.overlays.push(window);
    }

    let size = desktop.webview.inner_size();
    // Update the window metadata now that we've created a window
    let window_meta = WindowMeta {
        focused: desktop.is_focused(),
        maximized: desktop.is_maximized(),
        minimized: desktop.is_minimized(),
        minimal_view: size.width < 1200, // todo: why is it that on Linux, checking if desktop.inner_size().width < 600 is true?
    };
    state.ui.metadata = window_meta;

    use_shared_state_provider(cx, || state);
    use_shared_state_provider(cx, DownloadState::default);

    cx.render(rsx!(crate::app {}))
}

fn app(cx: Scope) -> Element {
    log::trace!("rendering app");
    let desktop = use_window(cx);
    let state = use_shared_state::<State>(cx)?;
    let download_state = use_shared_state::<DownloadState>(cx)?;

    // don't fetch friends and conversations from warp when using mock data
    let friends_init = use_ref(cx, || STATIC_ARGS.use_mock);
    let items_init = use_ref(cx, || STATIC_ARGS.use_mock);
    let chats_init = use_ref(cx, || STATIC_ARGS.use_mock);
    let needs_update = use_state(cx, || false);

    let mut font_style = String::new();
    if let Some(font) = state.read().ui.font.clone() {
        font_style = format!(
            "
        @font-face {{
            font-family: CustomFont;
            src: url('{}');
        }}
        body,
        html {{
            font-family: CustomFont, sans-serif;
        }}
        ",
            font.path
        );
    }

    // this gets rendered at the bottom. this way you don't have to scroll past all the use_futures to see what this function renders
    let main_element = {
        // render the Uplink app
        let user_lang_saved = state.read().settings.language.clone();
        change_language(user_lang_saved);

        let open_dyslexic =
            if state.read().configuration.general.dyslexia_support { ui::OPEN_DYSLEXIC } else { "" };

        let font_scale = format!("html {{ font-size: {}rem; }}", state.read().settings.font_scale());

        let theme = state.read().ui.theme.as_ref().map(|theme| theme.styles.clone()).unwrap_or_default();

        rsx! (
            style { "{UIKIT_STYLES} {APP_STYLE} {theme} {font_style} {open_dyslexic} {font_scale}" },
            div {
                id: "app-wrap",
                get_titlebar{},
                //get_toasts{},
                //get_call_dialog{},
                ui::get_pre_release_message{},
                get_router{},
                get_logger{},
            }
        )
    };

    // use_coroutine for software update

    // updates the UI
    let inner = download_state.inner();
    let updater_ch = use_coroutine(cx, |mut rx: UnboundedReceiver<SoftwareUpdateCmd>| {
        to_owned![needs_update];
        async move {
            while let Some(mut ch) = rx.next().await {
                while let Some(percent) = ch.0.recv().await {
                    if percent >= inner.borrow().read().progress + 5_f32 {
                        inner.borrow_mut().write().progress = percent;
                        needs_update.set(true);
                    }
                }
                inner.borrow_mut().write().stage = DownloadProgress::Finished;
                needs_update.set(true);
            }
        }
    });

    // receives a download command
    let _download_ch = use_coroutine(cx, |mut rx: UnboundedReceiver<SoftwareDownloadCmd>| {
        to_owned![updater_ch];
        async move {
            while let Some(dest) = rx.next().await {
                let (tx, rx) = mpsc::unbounded_channel::<f32>();
                updater_ch.send(SoftwareUpdateCmd(rx));
                match ui::utils::auto_updater::download_update(dest.0.clone(), tx).await {
                    Ok(downloaded_version) => {
                        log::debug!("downloaded version {downloaded_version}");
                    },
                    Err(e) => {
                        log::error!("failed to download update: {e}");
                    },
                }
            }
        }
    });

    // `use_future`s
    // all of Uplinks periodic tasks are located here. it's a lot to read but
    // it's better to have them in one place. this makes it a lot easier to find them.
    // there are 2 categories of tasks: warp tasks and UI tasks
    //
    // warp tasks
    // handle warp events
    // initialize friends: load from warp and store in State
    // initialize conversations: same
    //
    // UI tasks
    // clear toasts
    // update message timestamps
    // control child windows
    // clear typing indicator
    //
    // misc
    // when a task requires the UI be updated, `needs_update` is set.
    // when mock data is used, friends and conversations are generated randomly,
    //     not loaded from Warp. however, warp_runner continues to operate normally.
    //

    // yes, double render. sry.
    if *needs_update.get() {
        needs_update.set(false);
        state.write();
    }

    // There is currently an issue in Tauri/Wry where the window size is not reported properly.
    // Thus we bind to the resize event itself and update the size from the webview.
    let webview = desktop.webview.clone();
    let inner = state.inner();
    use_wry_event_handler(cx, {
        to_owned![needs_update, desktop];
        move |event, _| match event {
            WryEvent::WindowEvent { event: WindowEvent::Focused(focused), .. } => {
                //log::trace!("FOCUS CHANGED {:?}", *focused);
                if inner.borrow().read().ui.metadata.focused != *focused {
                    match inner.try_borrow_mut() {
                        Ok(state) => {
                            state.write().ui.metadata.focused = *focused;

                            if *focused {
                                state.write().ui.notifications.clear_badge();
                                let _ = state.write().save();
                            }
                            //crate::utils::sounds::Play(Sounds::Notification);
                            //needs_update.set(true);
                        },
                        Err(e) => {
                            log::error!("{e}");
                        },
                    }
                }
            },
            WryEvent::WindowEvent { event: WindowEvent::CloseRequested, .. } => match inner.try_borrow_mut() {
                Ok(state) => {
                    state.write().mutate(Action::ClearAllPopoutWindows(desktop.clone()));
                },
                Err(e) => log::error!("{e}"),
            },
            WryEvent::WindowEvent { event: WindowEvent::Resized(_), .. } => {
                let size = webview.inner_size();
                //log::trace!(
                //    "Resized - PhysicalSize: {:?}, Minimal: {:?}",
                //    size,
                //    size.width < 1200
                //);
                if desktop.outer_size().width < 575 {
                    desktop.set_title("");
                } else {
                    desktop.set_title("Uplink");
                }

                match inner.try_borrow_mut() {
                    Ok(state) => {
                        let metadata = state.read().ui.metadata.clone();
                        let new_metadata = WindowMeta { minimal_view: size.width < 600, ..metadata };
                        if metadata != new_metadata {
                            state.write().ui.sidebar_hidden = new_metadata.minimal_view;
                            state.write().ui.metadata = new_metadata;
                            needs_update.set(true);
                        }
                    },
                    Err(e) => {
                        log::error!("{e}");
                    },
                }
            },
            _ => {},
        }
    });

    // update state in response to warp events
    let inner = state.inner();
    use_future(cx, (), |_| {
        to_owned![needs_update, friends_init, chats_init];
        async move {
            // don't process warp events until friends and chats have been loaded
            while !(*friends_init.read() && *chats_init.read()) {
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            }
            let warp_event_rx = WARP_EVENT_CH.rx.clone();
            log::trace!("starting warp_runner use_future");
            // it should be sufficient to lock once at the start of the use_future. this is the only place the channel should be read from. in the off change that
            // the future restarts (it shouldn't), the lock should be dropped and this wouldn't block.
            let mut ch = warp_event_rx.lock().await;
            while let Some(evt) = ch.recv().await {
                match inner.try_borrow_mut() {
                    Ok(state) => {
                        state.write().process_warp_event(evt);
                        needs_update.set(true);
                    },
                    Err(e) => {
                        log::error!("{e}");
                    },
                }
            }
        }
    });

    // clear toasts
    let inner = state.inner();
    use_future(cx, (), |_| {
        to_owned![needs_update];
        async move {
            loop {
                sleep(Duration::from_secs(1)).await;
                match inner.try_borrow_mut() {
                    Ok(state) => {
                        if !state.read().has_toasts() {
                            continue;
                        }
                        if state.write().decrement_toasts() {
                            needs_update.set(true);
                        }
                    },
                    Err(e) => {
                        log::error!("{e}");
                    },
                }
            }
        }
    });

    // clear typing indicator
    let inner = state.inner();
    use_future(cx, (), |_| {
        to_owned![needs_update];
        async move {
            loop {
                sleep(Duration::from_secs(STATIC_ARGS.typing_indicator_timeout)).await;
                match inner.try_borrow_mut() {
                    Ok(state) => {
                        let now = Instant::now();
                        if state.write().clear_typing_indicator(now) {
                            needs_update.set(true);
                        }
                    },
                    Err(e) => {
                        log::error!("{e}");
                    },
                }
            }
        }
    });

    // periodically refresh message timestamps and friend's status messages
    use_future(cx, (), |_| {
        to_owned![needs_update];
        async move {
            loop {
                // simply triggering an update will refresh the message timestamps
                sleep(Duration::from_secs(60)).await;
                needs_update.set(true);
            }
        }
    });

    // check for updates
    let inner = state.inner();
    use_future(cx, (), |_| {
        to_owned![needs_update];
        async move {
            loop {
                let latest_release = match ui::utils::auto_updater::check_for_release().await {
                    Ok(opt) => match opt {
                        Some(r) => r,
                        None => {
                            sleep(Duration::from_secs(3600 * 24)).await;
                            continue;
                        },
                    },
                    Err(e) => {
                        log::error!("failed to check for release: {e}");
                        sleep(Duration::from_secs(3600 * 24)).await;
                        continue;
                    },
                };
                if inner.borrow().read().settings.update_dismissed == Some(latest_release.tag_name.clone()) {
                    sleep(Duration::from_secs(3600 * 24)).await;
                    continue;
                }
                match inner.try_borrow_mut() {
                    Ok(state) => {
                        state.write().update_available(latest_release.tag_name);
                        needs_update.set(true);
                    },
                    Err(e) => {
                        log::error!("{e}");
                    },
                }
                sleep(Duration::from_secs(3600 * 24)).await;
            }
        }
    });

    // control child windows
    let inner = state.inner();
    use_future(cx, (), |_| {
        to_owned![needs_update, desktop];
        async move {
            let window_cmd_rx = ui::WINDOW_CMD_CH.rx.clone();
            let mut ch = window_cmd_rx.lock().await;
            while let Some(cmd) = ch.recv().await {
                ui::window_manager::handle_cmd(inner.clone(), cmd, desktop.clone()).await;
                needs_update.set(true);
            }
        }
    });

    // initialize friends
    let inner = state.inner();
    use_future(cx, (), |_| {
        to_owned![friends_init, needs_update];
        async move {
            if *friends_init.read() {
                return;
            }
            let warp_cmd_tx = WARP_CMD_CH.tx.clone();
            let (tx, rx) =
                oneshot::channel::<Result<(friends::Friends, HashSet<state::Identity>), warp::error::Error>>();
            if let Err(e) = warp_cmd_tx.send(WarpCmd::MultiPass(MultiPassCmd::InitializeFriends { rsp: tx })) {
                log::error!("failed to initialize Friends {}", e);
                tokio::time::sleep(Duration::from_secs(1)).await;
                return;
            }

            let res = rx.await.expect("failed to get response from warp_runner");

            log::trace!("init friends");
            let friends = match res {
                Ok(friends) => friends,
                Err(e) => {
                    log::error!("init friends failed: {}", e);
                    return;
                },
            };

            match inner.try_borrow_mut() {
                Ok(state) => {
                    state.write().set_friends(friends.0, friends.1);
                    needs_update.set(true);
                },
                Err(e) => {
                    log::error!("{e}");
                },
            }

            *friends_init.write_silent() = true;
            needs_update.set(true);
        }
    });

    // initialize conversations
    let inner = state.inner();
    use_future(cx, (), |_| {
        to_owned![chats_init, needs_update];
        async move {
            if *chats_init.read() {
                return;
            }
            let warp_cmd_tx = WARP_CMD_CH.tx.clone();
            let res = loop {
                let (tx, rx) = oneshot::channel::<
                    Result<(HashMap<Uuid, state::Chat>, HashSet<state::Identity>), warp::error::Error>,
                >();
                if let Err(e) = warp_cmd_tx.send(WarpCmd::RayGun(RayGunCmd::InitializeConversations { rsp: tx })) {
                    log::error!("failed to init RayGun: {}", e);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    continue;
                }

                match rx.await {
                    Ok(r) => break r,
                    Err(e) => {
                        log::error!("command canceled: {}", e);
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await
                    },
                }
            };

            log::trace!("init chats");
            let chats = match res {
                Ok(r) => r,
                Err(e) => {
                    log::error!("failed to initialize chats: {}", e);
                    return;
                },
            };

            match inner.try_borrow_mut() {
                Ok(state) => {
                    state.write().set_chats(chats.0, chats.1);
                    needs_update.set(true);
                },
                Err(e) => {
                    log::error!("{e}");
                },
            }

            *chats_init.write_silent() = true;
            needs_update.set(true);
        }
    });

    cx.render(main_element)
}

fn get_update_icon(cx: Scope) -> Element {
    log::trace!("rendering get_update_icon");
    let state = use_shared_state::<State>(cx)?;
    let download_state = use_shared_state::<DownloadState>(cx)?;
    let desktop = use_window(cx);
    let download_ch = use_coroutine_handle::<SoftwareDownloadCmd>(cx)?;

    let new_version = match state.read().settings.update_available.as_ref() {
        Some(u) => u.clone(),
        None => return cx.render(rsx!("")),
    };

    let update_msg = format!("{}: {}", get_local_text("uplink.update-available"), new_version,);
    let downloading_msg =
        format!("{}: {}%", get_local_text("uplink.update-downloading"), download_state.read().progress as u32);
    let downloaded_msg = get_local_text("uplink.update-downloaded");

    let stage = download_state.read().stage;
    match stage {
        DownloadProgress::Idle => cx.render(rsx!(
            ContextMenu {
                key: "update-available-menu",
                id: "update-available-menu".to_string(),
                items: cx.render(rsx!(
                    ContextItem {
                        text: get_local_text("uplink.update-menu-dismiss"),
                        onpress: move |_| {
                            state.write().mutate(Action::DismissUpdate);
                        }
                    },
                    ContextItem {
                        text: get_local_text("uplink.update-menu-download"),
                        onpress: move |_| {
                            if let Some(dest) = get_download_dest() {
                                download_state.write().stage = DownloadProgress::Pending;
                                download_state.write().destination = Some(dest.clone());
                                download_ch.send(SoftwareDownloadCmd(dest));
                            }
                        }
                    }
                )),
                div {
                    id: "update-available",
                    aria_label: "update-available",
                    onclick: move |_| {
                        if let Some(dest) = get_download_dest() {
                            download_state.write().stage = DownloadProgress::Pending;
                            download_state.write().destination = Some(dest.clone());
                            download_ch.send(SoftwareDownloadCmd(dest));
                        }
                    },
                    IconElement {
                        icon: common::icons::solid::Shape::ArrowDown,
                        fill: "green",
                    },
                    "{update_msg}",
                }
            }
        )),
        DownloadProgress::Pending => cx.render(rsx!(div {
            id: "update-available",
            aria_label: "update-available",
            "{downloading_msg}"
        })),
        DownloadProgress::Finished => {
            cx.render(rsx!(div {
                id: "update-available",
                aria_label: "update-available",
                onclick: move |_| {
                    // be sure to update this before closing the app
                    state.write().mutate(Action::DismissUpdate);
                    if let Some(dest) = download_state.read().destination.clone() {
                        std::thread::spawn(move ||  {

                            let cmd = if cfg!(target_os = "windows") {
                                "explorer"
                            } else if cfg!(target_os = "linux") {
                                "xdg-open"
                            } else if cfg!(target_os = "macos") {
                                "open"
                            } else {
                               eprintln!("unknown OS type. failed to open files browser");
                               return;
                            };
                            Command::new(cmd)
                            .arg(dest)
                            .spawn()
                            .unwrap();
                        });
                        desktop.close();
                    } else {
                        log::error!("attempted to download update without download location");
                    }
                    download_state.write().destination = None;
                    download_state.write().stage = DownloadProgress::Idle;
                },
                "{downloaded_msg}"
            }))
        },
    }
}

fn get_logger(cx: Scope) -> Element {
    let state = use_shared_state::<State>(cx)?;

    cx.render(rsx!(state.read().configuration.developer.developer_mode.then(|| rsx!(DebugLogger {}))))
}

fn get_toasts(cx: Scope) -> Element {
    let state = use_shared_state::<State>(cx)?;
    cx.render(rsx!(state.read().ui.toast_notifications.iter().map(|(id, toast)| {
        rsx!(Toast {
            id: *id,
            with_title: toast.title.clone(),
            with_content: toast.content.clone(),
            icon: toast.icon.unwrap_or(Icon::InformationCircle),
            appearance: Appearance::Secondary,
        },)
    })))
}

#[allow(unused_assignments)]
fn get_titlebar(cx: Scope) -> Element {
    let desktop = use_window(cx);
    let state = use_shared_state::<State>(cx)?;
    let config = state.read().configuration.clone();

    #[allow(unused_mut)]
    let mut controls: Option<VNode> = None;

    #[cfg(not(target_os = "macos"))]
    {
        controls = cx.render(rsx!(
            div {
                class: "controls",
                Button {
                    aria_label: "minimize-button".into(),
                    icon: Icon::Minus,
                    appearance: Appearance::Transparent,
                    onpress: move |_| {
                        desktop.set_minimized(true);
                    }
                },
                Button {
                    aria_label: "square-button".into(),
                    icon: Icon::Square2Stack,
                    appearance: Appearance::Transparent,
                    onpress: move |_| {
                        desktop.set_maximized(!desktop.is_maximized());
                    }
                },
                Button {
                    aria_label: "close-button".into(),
                    icon: Icon::XMark,
                    appearance: Appearance::Transparent,
                    onpress: move |_| {
                        desktop.close();
                    }
                },
            }
        ))
    }

    cx.render(rsx!(
        div {
            id: "titlebar",
            onmousedown: move |_| { desktop.drag(); },
            get_update_icon{},
            // Only display this if developer mode is enabled.
            (config.developer.developer_mode).then(|| rsx!(
                Button {
                    aria_label: "device-phone-mobile-button".into(),
                    icon: Icon::DevicePhoneMobile,
                    appearance: Appearance::Transparent,
                    onpress: move |_| {
                        desktop.set_inner_size(LogicalSize::new(300.0, 534.0));
                        let meta = state.read().ui.metadata.clone();
                        state.write().mutate(Action::SetMeta(WindowMeta {
                            minimal_view: true,
                            ..meta
                        }));
                        state.write().mutate(Action::SidebarHidden(true));
                        state.write().mock_own_platform(Platform::Mobile);
                    }
                },
                Button {
                    aria_label: "device-tablet-button".into(),
                    icon: Icon::DeviceTablet,
                    appearance: Appearance::Transparent,
                    onpress: move |_| {
                        desktop.set_inner_size(LogicalSize::new(600.0, 534.0));
                        let meta = state.read().ui.metadata.clone();
                        state.write().mutate(Action::SetMeta(WindowMeta {
                            minimal_view: false,
                            ..meta
                        }));
                        state.write().mutate(Action::SidebarHidden(false));
                        state.write().mock_own_platform(Platform::Web);
                    }
                },
                Button {
                    aria_label: "computer-desktop-button".into(),
                    icon: Icon::ComputerDesktop,
                    appearance: Appearance::Transparent,
                    onpress: move |_| {
                        desktop.set_inner_size(LogicalSize::new(950.0, 600.0));
                        let meta = state.read().ui.metadata.clone();
                        state.write().mutate(Action::SetMeta(WindowMeta {
                            minimal_view: false,
                            ..meta
                        }));
                        state.write().mutate(Action::SidebarHidden(false));
                        state.write().mock_own_platform(Platform::Desktop);
                    }
                },
                Button {
                    aria_label: "command-line-button".into(),
                    icon: Icon::CommandLine,
                    appearance: Appearance::Transparent,
                    onpress: |_| {
                        desktop.devtool();
                    }
                }
            )),

            controls,

        },
    ))
}

fn get_call_dialog(_cx: Scope) -> Element {
    // CallDialog {
    //     caller: cx.render(rsx!(UserImage {
    //         platform: Platform::Mobile,
    //         status: Status::Online
    //     })),
    //     callee: cx.render(rsx!(UserImage {
    //         platform: Platform::Mobile,
    //         status: Status::Online
    //     })),
    //     description: "Call Description".into(),
    //     // with_accept_btn: cx.render(rsx! (
    //     //     Button {
    //     //         icon: Icon::Phone,
    //     //         appearance: Appearance::Success,
    //     //     }
    //     // )),
    //     with_deny_btn: cx.render(rsx! (
    //         Button {
    //             icon: Icon::PhoneXMark,
    //             appearance: Appearance::Danger,
    //             text: "End".into(),
    //         }
    //     )),
    // }
    None
}

fn get_router(cx: Scope) -> Element {
    let state = use_shared_state::<State>(cx)?;
    let pending_friends = state.read().friends().incoming_requests.len();

    let chat_route = UIRoute {
        to: ui::UPLINK_ROUTES.chat,
        name: get_local_text("uplink.chats"),
        icon: Icon::ChatBubbleBottomCenterText,
        ..UIRoute::default()
    };
    let settings_route = UIRoute {
        to: ui::UPLINK_ROUTES.settings,
        name: get_local_text("settings.settings"),
        icon: Icon::Cog6Tooth,
        ..UIRoute::default()
    };
    let friends_route = UIRoute {
        to: ui::UPLINK_ROUTES.friends,
        name: get_local_text("friends.friends"),
        icon: Icon::Users,
        with_badge: if pending_friends > 0 { Some(pending_friends.to_string()) } else { None },
        loading: None,
    };
    let files_route = UIRoute {
        to: ui::UPLINK_ROUTES.files,
        name: get_local_text("files.files"),
        icon: Icon::Folder,
        ..UIRoute::default()
    };
    let routes = vec![
        chat_route.clone(),
        files_route.clone(),
        friends_route.clone(),
        settings_route.clone(),
    ];

    cx.render(rsx!(
        Router {
            Route {
                to: ui::UPLINK_ROUTES.loading,
                LoadingLayout{}
            },
            Route {
                to: ui::UPLINK_ROUTES.chat,
                ChatLayout {
                    route_info: RouteInfo {
                        routes: routes.clone(),
                        active: chat_route.clone(),
                    }
                }
            },
            Route {
                to: ui::UPLINK_ROUTES.settings,
                SettingsLayout {
                    route_info: RouteInfo {
                        routes: routes.clone(),
                        active: settings_route.clone(),
                    }
                }
            },
            Route {
                to: ui::UPLINK_ROUTES.friends,
                FriendsLayout {
                    route_info: RouteInfo {
                        routes: routes.clone(),
                        active: friends_route.clone(),
                    }
                }
            },
            Route {
                to: ui::UPLINK_ROUTES.files,
                FilesLayout {
                    route_info: RouteInfo {
                        routes: routes.clone(),
                        active: files_route,
                    }
                }
            }
        }
    ))
}
