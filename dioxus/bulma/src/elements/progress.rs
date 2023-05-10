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

// #[allow(non_snake_case)]
// pub fn ProgressTimer<'a>(cx: Scope<'a, ProgressProps<'a>>) -> Element {
//     let state = cx.props.state;

//     // Fork a background function to track progress
//     cx.render(rsx! {
//         Progress {
//             state: state,
//             max: cx.props.max,
//             value: cx.props.max,
//         }
//     })
// }
