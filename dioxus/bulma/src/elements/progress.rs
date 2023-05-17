//! Provides progress
use crate::utils::*;
use dioxus::prelude::Scoped;
use dioxus::prelude::*;
use fermi::{use_atom_ref, use_atom_root, AtomRef, Readable, UseAtomRef};
use instant::Instant;

const RESOLUTION: u64 = 500;
const MIN_INTERVAL_MS: u64 = 50;
const DEFAULT_DURATION_MS: u64 = 15000;

static SIGNAL: AtomRef<bool> = |_| false;

/// Progress shared state
#[derive(Clone)]
pub struct Progress {
    // Progress identifier
    id: String,

    // Max progress value
    max: f64,

    // Current progress value
    value: f64,

    // Track if the progress has been started
    running: bool,

    // Optional signal to trigger
    signal: Option<UseAtomRef<bool>>,

    // Track if progress completion has been signaled
    signaled: bool,

    // Instant the progress timer started
    start: Option<Instant>,

    // Duration of the progress bar in milliseconds if timed
    duration: u64,
}

impl Default for Progress {
    fn default() -> Self {
        Self {
            id: String::new(),
            max: 1.0,
            value: 0.0,
            running: false,
            signal: None,
            signaled: false,
            start: None,
            duration: DEFAULT_DURATION_MS,
        }
    }
}

impl Progress {
    /// Subscribe to the progress completion notification which will trigger the
    /// caller to re-render when the progress is completed
    /// * `cx: &Scoped` Dioxus scoped state
    /// * `atom: &AtomRef` Dioxus scoped state
    pub fn subscribe(cx: &Scoped, atom: AtomRef<Progress>) {
        let atom_ref = use_atom_ref(&cx, atom);
        println!("after atom_ref");

        // Unsubscribe before making changes to avoid triggering renders
        use_atom_root(cx).unsubscribe(atom.unique_id(), cx.scope_id());
        println!("after atom_root");

        let signal = use_atom_ref(cx, SIGNAL);
        println!("after signal");

        atom_ref.write_silent().with_notify(signal.clone());
        println!("after with_notify");

        atom_ref.write_silent().reset();
        println!("after reset");
    }

    /// Set the progress identifier
    /// * `id: &str` progress identifier
    pub fn with_id(&mut self, id: &str) -> &mut Self {
        self.id = id.into();
        self
    }

    /// Subscribe to the completion notification using the given custom signal.
    /// The given signal will be triggered causing a re-render event to the owner.
    /// when the progress is completed.
    /// * `signal: UseAtomRef<bool>` fermi atom used as a signal
    pub fn with_notify(&mut self, signal: UseAtomRef<bool>) {
        self.signal = Some(signal);
    }

    /// Start or restart progress
    /// * `id: &str` progress identifier
    /// * `max: f64` progress maximum value
    /// * `value: f64` progress current value
    /// * `signal: Option<UseAtomRef<bool>>` is an optional signal to send out to listeners
    pub fn start(&mut self, id: &str, max: f64, value: f64, signal: Option<UseAtomRef<bool>>) {
        self.running = true;
        self.signal = signal;
        self.signaled = false;
        self.id = id.to_string();
        self.max = max;
        self.value = value;
    }

    /// Start or restart progress
    /// * `id: &str` id for creating or resetting progress
    /// * `duration: u64` milliseconds to wait before progress is complete
    /// * `signal: Option<UseAtomRef<bool>>` is an optional signal to send out to listeners
    pub fn timed(&mut self, id: &str, duration: u64, signal: Option<UseAtomRef<bool>>) {
        self.running = true;
        // self.signal = signal;
        self.signaled = false;
        self.id = id.to_string();
        self.start = Some(Instant::now());
        self.duration = duration;
    }

    /// Advance the timed progress
    /// * `returns: bool` true if completed
    pub fn advance(&mut self) -> bool {
        let mut result = false;
        if self.running {
            if let Some(start) = self.start {
                if self.value < self.max {
                    let elapsed = start.elapsed().as_millis() as u64;
                    self.value = elapsed as f64 / self.duration as f64;
                }
                if self.value >= self.max {
                    self.complete();
                    result = true;
                }
            }
        }
        result
    }

    /// Complete the progress
    pub fn complete(&mut self) {
        self.value = self.max;
        self.signal();
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

    /// Get the progress identifier
    /// * returns `id: &str` progress identifier
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the progress timer interval
    /// * returns `interval: u64` milliseconds to wait before firing
    pub fn interval(&self) -> u64 {
        (self.duration / RESOLUTION).min(MIN_INTERVAL_MS)
    }

    /// Reset the progress value
    pub fn reset(&mut self) {
        self.value = 0.0;
        self.running = false;
        self.signaled = false;
        if self.start.is_some() {
            self.start = Some(Instant::now());
        }
    }

    /// Check if progress is running
    /// * returns `true` when progress has been started using the start method
    pub fn running(&self) -> bool {
        self.running
    }

    /// Set the progress value
    /// * `value: f64` the value to set
    pub fn set(&mut self, value: f64) {
        self.value = value;
        if self.value >= self.max {
            self.complete();
        }
    }

    /// Set the signaled status to true
    fn signal(&mut self) {
        if !self.signaled {
            self.signaled = true;
            if let Some(signal) = &self.signal {
                signal.set(true);
            }
        }
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
    #[props(!optional)]
    id: String,

    #[props(default = 1.0)]
    max: f64,

    #[props(default = 0.0)]
    value: f64,

    #[props(optional)]
    size: Option<Sizes>,

    #[props(optional)]
    color: Option<Colors>,

    #[props(!optional)]
    state: &'a UseAtomRef<Progress>,

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
        state.write().start(
            &cx.props.id,
            cx.props.max,
            cx.props.value,
            cx.props.completed.and_then(|x| Some(x.clone())),
        );
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
    state: &'a UseAtomRef<Progress>,

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
    log::trace!("ProgressTimed[{}]: rendered", cx.props.id);
    let state = cx.props.state;

    // Configure timed progress
    if !state.read().running() {
        log::debug!("ProgressTimed[{}]: created", cx.props.id);
        state.write().timed(&cx.props.id, cx.props.duration, cx.props.completed.and_then(|x| Some(x.clone())));
    }
    let (max, value) = state.read().values();

    // Submit future to the Dioxus scheduler which only allows one instance at a time.
    // When the `id` value changes the future will be regenerated to include the new values.
    use_future(&cx, &cx.props.id, |id| {
        to_owned![state];
        log::debug!("Future[{}]: created", &id);
        async move {
            loop {
                sleep(state.read().interval()).await;
                if state.write().advance() {
                    break;
                }
            }
            log::debug!("Future[{}]: completed", &id);
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
