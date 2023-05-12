use crate::{state::*, utils::*};
use dioxus::prelude::*;
use fermi::{use_atom_ref, AtomRef, UseAtomRef};
use instant::Instant;

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
    state: &'a UseAtomRef<ProgressState>,

    #[props(optional)]
    completed: Option<&'a UseAtomRef<bool>>,
}

/// Progress bar
///
/// ### Properties
/// * `id: String` id used for progress state lookup
/// * `max: f64` max value for the progress bar, defaults to 1.0
/// * `value: f64` current value of the progress bar, defaults to 0.0
/// * `size: Option<Sizes>` optional CSS size of the progress bar
/// * `color: Option<Colors>` optional CSS color of the progress bar
/// * `state: &'a UseAtomRef<ProgressState>` fermi state reference for progress tracking
#[allow(non_snake_case)]
pub fn Progress<'a>(cx: Scope<'a, ProgressProps<'a>>) -> Element {
    let state = cx.props.state;

    // Ensure progress has been configured
    if !state.read().running() {
        state.write().start(cx.props.id, cx.props.max, cx.props.value);
    }
    let (max, value) = state.read().values();

    // Set completion signal
    if state.read().completed() && !state.read().signaled() {
        if let Some(completed) = cx.props.completed {
            completed.set(state.write().signal());
        }
    }

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

    #[props(default = 15000)]
    duration: u64,

    #[props(optional)]
    size: Option<Sizes>,

    #[props(optional)]
    color: Option<Colors>,

    #[props(!optional)]
    state: &'a UseAtomRef<ProgressState>,

    #[props(optional)]
    completed: Option<&'a UseAtomRef<bool>>,
}

/// Timed progress bar provides will automatically increment every 50ms until it hits 100%
/// of the specified duration.  Unique ids must be used or you'll have cross timer updates
/// happening
///
/// ### Properties
/// * `id: String` id used for progress state lookup
/// * `duration: usize` milliseconds to wait before completing the progress bar, default 15000
/// * `size: Option<Sizes>` optional CSS size of the progress bar
/// * `color: Option<Colors>` optional CSS color of the progress bar
/// * `state: &'a UseAtomRef<ProgressState>` fermi state reference for progress tracking
/// * `completed: Option<&'a UseAtomRef<bool>>` optional completed signal for listeners
#[allow(non_snake_case)]
pub fn ProgressTimed<'a>(cx: Scope<'a, ProgressTimedProps<'a>>) -> Element {
    println!("render: progress timed: {}", cx.props.id);
    let state = cx.props.state;

    // Configure timed progress
    if !state.read().running() {
        state.write().timed(cx.props.id, cx.props.duration);
    }
    let (max, value) = state.read().values();

    // Submit to Dioxus scheduler which only allows one instance of this future at a time
    let future = use_future(&cx, (), |_| {
        to_owned![state];
        let id = cx.props.id.to_string();
        println!("future: {}", &id);
        let interval = state.read().interval();
        let completed = cx.props.completed.and_then(|x| Some(x.clone()));
        async move {
            loop {
                sleep(interval).await;
                if state.write().advance() {
                    if !state.read().signaled() {
                        if let Some(signal) = completed {
                            signal.set(state.write().signal());
                        }
                    }
                    //cx.props.oncomplete.as_ref().map(|x| x.call(()));
                    break;
                }
            }
            println!("goodbye: {}", &id);
        }
    });

    // If the future has commpleted then cancel it to be recreated next time
    // ProgressTimed out is called
    if future.value().is_some() {
        //
    }

    // Configure CSS class
    let mut class = "progress".to_string();
    if cx.props.size.is_some() {
        class += &format!(" is-{}", cx.props.size.as_ref().unwrap().to_string());
    }
    if cx.props.color.is_some() {
        class += &format!(" is-{}", cx.props.color.as_ref().unwrap().to_string());
    }

    // Render progress bar
    cx.render(rsx! {
        progress {
            class: "{class}",
            max: "{max}",
            value: "{value}",
        }
    })
}

#[cfg(target_family = "wasm")]
async fn sleep(interval: u64) {
    gloo_timers::future::TimeoutFuture::new(interval as u32).await;
}

#[cfg(any(windows, unix))]
async fn sleep(interval: u64) {
    tokio::time::sleep(tokio::time::Duration::from_millis(interval)).await;
}
