use crate::{state::GlobalState, utils::*};
use dioxus::prelude::*;
use fermi::UseAtomRef;

#[allow(non_snake_case)]
#[derive(Props)]
pub struct ProgressProps<'a> {
    #[props(!optional)]
    id: &'a str,

    #[props(default = 1.0)]
    max: f64,

    #[props(default = 0.0)]
    value: f64,

    #[props(optional)]
    size: Option<Sizes>,

    #[props(optional)]
    color: Option<Colors>,

    #[props(!optional)]
    state: &'a UseAtomRef<GlobalState>,
}

/// Progress bar
///
/// ### Properties
/// * `id: String` id used for progress state lookup
/// * `max: f64` max value for the progress bar, defaults to 1.0
/// * `value: f64` current value of the progress bar, defaults to 0.0
/// * `size: Option<Sizes>` optional CSS size of the progress bar
/// * `color: Option<Colors>` optional CSS color of the progress bar
/// * `state: &'a UseAtomRef<GlobalState>` global fermi state reference for tracking
#[allow(non_snake_case)]
pub fn Progress<'a>(cx: Scope<'a, ProgressProps<'a>>) -> Element {
    let state = cx.props.state;

    // Ensure progress has been configured
    if !state.read().progress.exists(cx.props.id) {
        state.write().progress.new(cx.props.id, cx.props.max, cx.props.value);
    }
    let (max, value) = state.read().progress.get(cx.props.id);

    // Configure class
    let mut class = "progress".to_string();
    if cx.props.size.is_some() {
        class += &format!(" is-{}", cx.props.size.as_ref().unwrap().to_string());
    }
    if cx.props.color.is_some() {
        class += &format!(" is-{}", cx.props.color.as_ref().unwrap().to_string());
    }

    cx.render(rsx! {
        progress {
            class: "{class}",
            max: "{max}",
            value: "{value}",
        }
    })
}

#[allow(non_snake_case)]
#[derive(Props)]
pub struct ProgressTimedProps<'a> {
    #[props(!optional)]
    id: &'a str,

    #[props(default = 250)]
    interval: usize,

    #[props(default = 0.01)]
    increment: f64,

    #[props(default = 1.0)]
    max: f64,

    #[props(default = 0.0)]
    value: f64,

    #[props(optional)]
    size: Option<Sizes>,

    #[props(optional)]
    color: Option<Colors>,

    #[props(!optional)]
    state: &'a UseAtomRef<GlobalState>,
}

/// Progress bar
///
/// ### Properties
/// * `id: String` id used for progress state lookup
/// * `interval: usize` milliseconds to wait before advancing the progress bar
/// * `increment: f64` quantity to increment the progress bar by
/// * `max: f64` max value for the progress bar, defaults to 1.0
/// * `value: f64` current value of the progress bar, defaults to 0.0
/// * `size: Option<Sizes>` optional CSS size of the progress bar
/// * `color: Option<Colors>` optional CSS color of the progress bar
/// * `state: &'a UseAtomRef<GlobalState>` global fermi state reference for tracking
#[allow(non_snake_case)]
pub fn ProgressTimed<'a>(cx: Scope<'a, ProgressTimedProps<'a>>) -> Element {
    let state = cx.props.state;

    // Ensure timed progress has been configured
    if !state.read().progress.exists(cx.props.id) {
        state.write().progress.timed(
            cx.props.id,
            cx.props.max,
            cx.props.value,
            cx.props.increment,
            cx.props.interval,
        );
    }
    let (max, value) = state.read().progress.get(cx.props.id);

    // Spawn background thread to time the progress
    use_future(&cx, (), |_| {
        let id = cx.props.id;
        let state = state.clone();
        async move {
            loop {
                // if let Some(hide_after) = item.hide_after {
                //     if chrono::Local::now().timestamp() >= hide_after {
                //         toast_manager.write().list.remove(id);
                //     }
                // }
                // time_sleep(100).await;
            }
        }
    });

    // Configure class
    let mut class = "progress".to_string();
    if cx.props.size.is_some() {
        class += &format!(" is-{}", cx.props.size.as_ref().unwrap().to_string());
    }
    if cx.props.color.is_some() {
        class += &format!(" is-{}", cx.props.color.as_ref().unwrap().to_string());
    }

    cx.render(rsx! {
        progress {
            class: "{class}",
            max: "{max}",
            value: "{value}",
        }
    })
}

#[cfg(target_family = "wasm")]
async fn time_sleep(interval: usize) {
    gloo_timers::future::TimeoutFuture::new(interval as u32).await;
}

// #[cfg(any(windows, unix))]
// async fn time_sleep(interval: usize) {
//     tokio::time::sleep(tokio::time::Duration::from_millis(interval as u64)).await;
// }
