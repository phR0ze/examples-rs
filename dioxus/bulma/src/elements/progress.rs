//! Provides progress
use crate::utils::*;
use dioxus::prelude::*;
use fermi::{use_atom_ref, AtomRef, UseAtomRef};
use instant::Instant;

const RESOLUTION: u64 = 500;
const MIN_INTERVAL_MS: u64 = 50;
const DEFAULT_DURATION_MS: u64 = 15000;

/// Progress shared state
#[derive(Clone)]
pub struct Progress {
    // Max progress value
    max: f64,

    // Current progress value
    value: f64,

    // Pause the progress
    pause: bool,

    // Optional signal to trigger
    signal: Option<UseAtomRef<bool>>,

    // Track if progress completion has been signaled
    signaled: bool,

    // Instant the progress timer started
    started: Option<Instant>,

    // Duration of the progress bar in milliseconds if timed
    duration: u64,
}

impl Default for Progress {
    fn default() -> Self {
        Self {
            max: 1.0,
            value: 0.0,
            pause: false,
            signal: None,
            signaled: false,
            started: None,
            duration: DEFAULT_DURATION_MS,
        }
    }
}

impl Progress {
    /// Start or restart progress
    /// * `max: f64` progress maximum value
    /// * `value: f64` progress current value
    /// * `signal: Option<UseAtomRef<bool>>` is an optional signal to send out to listeners
    pub fn start(&mut self, max: f64, value: f64, signal: Option<UseAtomRef<bool>>) {
        self.signal = signal;
        self.signaled = false;
        self.max = max;
        self.started = Some(Instant::now());
        self.value = value;
    }

    /// Start or restart progress
    /// * `duration: u64` milliseconds to wait before progress is complete
    /// * `signal: Option<UseAtomRef<bool>>` is an optional signal to send out to listeners
    pub fn timed(&mut self, duration: u64, signal: Option<UseAtomRef<bool>>) {
        self.signal = signal;
        self.signaled = false;
        self.started = Some(Instant::now());
        self.duration = duration;
    }

    /// Advance the timed progress
    /// * `returns: bool` true if completed
    pub fn advance(&mut self) -> bool {
        let mut result = false;
        if let Some(started) = self.started {
            if self.value < self.max {
                let elapsed = started.elapsed().as_millis() as u64;
                self.value = elapsed as f64 / self.duration as f64;
            }
            if self.value >= self.max {
                self.complete();
                result = true;
            }
        }
        result
    }

    /// Complete the progress
    pub fn complete(&mut self) {
        self.value = self.max;
        self.started = None;
        if !self.signaled {
            self.signaled = true;
            if let Some(signal) = &self.signal {
                signal.set(true);
            }
        }
    }

    /// Check if the progress bar is completed
    /// * returns `completed: bool` true if completed
    pub fn completed(&self) -> bool {
        self.value >= self.max
    }

    /// Get the progress duration
    /// * returns `duration: u64`
    pub fn duration(&self) -> u64 {
        self.duration
    }

    /// Pause progress from advancing
    pub fn pause(&mut self) {
        self.pause = true;
    }

    /// Get the progress timer interval
    /// * returns `interval: u64` milliseconds to wait before firing
    pub fn interval(&self) -> u64 {
        (self.duration / RESOLUTION).min(MIN_INTERVAL_MS)
    }

    /// Reset the progress value
    pub fn reset(&mut self) {
        self.value = 0.0;
        self.signaled = false;
        self.started = None;
    }

    /// Resume progress advancing if paused
    pub fn resume(&mut self) {
        self.pause = false;
    }

    /// Running if started and not yet completed and not paused
    /// * returns `bool`
    pub fn running(&self) -> bool {
        self.started() && !self.completed() && !self.pause
    }

    /// Set the progress value
    /// * `value: f64` the value to set
    pub fn set(&mut self, value: f64) {
        self.value = value;
        if self.value >= self.max {
            self.complete();
        }
    }

    /// Determine if the progress is started or not
    /// * returns `bool` true if started
    pub fn started(&self) -> bool {
        self.started.is_some()
    }

    /// Get the progress value
    /// * returns `value: f64`
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Get the progress max and value
    /// * returns `(max: f64, value: f64)`
    pub fn values(&self) -> (f64, f64) {
        (self.max, self.value)
    }
}

#[allow(non_snake_case)]
#[derive(Props)]
pub struct ProgressProps<'a> {
    #[props(default = 1.0)]
    max: f64,

    #[props(default = 0.0)]
    value: f64,

    #[props(optional)]
    size: Option<Sizes>,

    #[props(optional)]
    color: Option<Colors>,

    #[props(!optional)]
    state: AtomRef<Progress>,

    #[props(optional)]
    completed: Option<&'a UseAtomRef<bool>>,
}

/// Progress bar
///
/// ### Detail
/// Progress specifically uses the `AtomRef` form of state to avoid forcing the parent
/// to receive render updates if not desired
///
/// ### Properties
/// * `max: f64` max value for the progress bar, defaults to 1.0
/// * `value: f64` current value of the progress bar, defaults to 0.0
/// * `size: Option<Sizes>` optional CSS size of the progress bar
/// * `color: Option<Colors>` optional CSS color of the progress bar
/// * `state: AtomRef<Progress>` fermi state reference for progress tracking
/// * `completed: Option<&'a UseAtomRef<bool>>` optional completed signal for listeners
#[allow(non_snake_case)]
pub fn Progress<'a>(cx: Scope<'a, ProgressProps<'a>>) -> Element {
    let state = use_atom_ref(cx, cx.props.state);

    // Ensure progress has been configured
    // By checking if it has been completed here we don't automatically restart the progress
    // if started gets reset to None on complete. This keeps the restart an intentional action.
    if !state.read().started() && !state.read().completed() {
        state.write().start(cx.props.max, cx.props.value, cx.props.completed.and_then(|x| Some(x.clone())));
    }
    let (max, value) = state.read().values();

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
    id: String,

    #[props(default = 15000)]
    duration: u64,

    #[props(optional)]
    size: Option<Sizes>,

    #[props(optional)]
    color: Option<Colors>,

    #[props(!optional)]
    state: AtomRef<Progress>,

    #[props(optional)]
    completed: Option<&'a UseAtomRef<bool>>,
}

/// Timed progress bar provides will automatically increment every 50ms until it hits 100%
/// of the specified duration.  Unique ids must be used or you'll have cross timer updates
/// happening
//
/// ### Detail
/// Progress specifically uses the `AtomRef` form of state to avoid forcing the parent
/// to receive render updates if not desired. Using `UseAtomRef` for the signal for the opposite
/// reason to make registering for events convenient.
///
/// ### Properties
/// * `id: String` progress restart will occur every time the id changes
/// * `duration: usize` milliseconds to wait before completing the progress bar, default 15000
/// * `size: Option<Sizes>` optional CSS size of the progress bar
/// * `color: Option<Colors>` optional CSS color of the progress bar
/// * `state: AtomRef<Progress>` fermi state reference for progress tracking
/// * `completed: Option<&'a UseAtomRef<bool>>` optional completed signal for listeners
#[allow(non_snake_case)]
pub fn ProgressTimed<'a>(cx: Scope<'a, ProgressTimedProps<'a>>) -> Element {
    log::trace!("ProgressTimed[{}]: rendered", cx.props.id);
    let state = use_atom_ref(cx, cx.props.state);

    // Configure timed progress
    // By checking if it has been completed here we don't automatically restart the progress
    // if started gets reset to None on complete. This keeps the restart an intentional action.
    if !state.read().started() && !state.read().completed() {
        log::debug!("ProgressTimed[{}]: started", cx.props.id);
        state.write().timed(cx.props.duration, cx.props.completed.and_then(|x| Some(x.clone())));
    }
    let (max, value) = state.read().values();

    // Submit future to the Dioxus scheduler which only allows one instance at a time.
    // When the `id` value changes the future will be regenerated to include the new values.
    use_future(&cx, &cx.props.id, |id| {
        to_owned![state];
        log::info!("Future[{}]: created", id);
        async move {
            loop {
                sleep(state.read().interval()).await;
                if state.write().advance() {
                    break;
                }
            }
            log::info!("Future[{}]: completed", id);
        }
    });

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
