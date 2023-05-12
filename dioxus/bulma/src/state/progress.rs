//! Provides progress shared state
use instant::Instant;

const RESOLUTION: u64 = 500;
const MIN_INTERVAL_MS: u64 = 50;
const DEFAULT_DURATION_MS: u64 = 15000;

/// Progress shared state
#[derive(Clone)]
pub struct ProgressState {
    // Progress identifier
    id: String,

    // Max progress value
    max: f64,

    // Current progress value
    value: f64,

    // Track if the progress has been started
    running: bool,

    // Track if progress completion has been signaled
    signaled: bool,

    // Instant the progress timer started
    start: Option<Instant>,

    // Duration of the progress bar in milliseconds if timed
    duration: u64,
}

impl Default for ProgressState {
    fn default() -> Self {
        Self {
            id: String::new(),
            max: 1.0,
            value: 0.0,
            running: false,
            signaled: false,
            start: None,
            duration: 15000,
        }
    }
}

impl ProgressState {
    /// Start or restart progress
    /// * `id: &str` progress identifier
    /// * `max: f64` progress maximum value
    /// * `value: f64` progress current value
    pub fn start(&mut self, id: &str, max: f64, value: f64) {
        self.running = true;
        self.signaled = false;
        self.id = id.to_string();
        self.max = max;
        self.value = value;
    }

    /// Start or restart progress
    /// * `id: &str` id for creating or resetting progress
    /// * `duration: u64` milliseconds to wait before progress is complete
    pub fn timed(&mut self, id: &str, duration: u64) {
        self.running = true;
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
                    self.value = self.max;
                    result = true;
                }
            }
        }
        result
    }

    /// Complete the progress
    pub fn complete(&mut self) {
        self.value = self.max;
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

    /// Get the progress timer interval
    /// * returns `interval: u64` milliseconds to wait before firing
    pub fn interval(&self) -> u64 {
        (self.duration / RESOLUTION).min(MIN_INTERVAL_MS)
    }

    /// Reset the progress value
    pub fn reset(&mut self) {
        self.value = 0.0;
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
            self.value = self.max;
        }
    }

    /// Check if progress already signaled completion
    /// * returns `true` if already signaled
    pub fn signaled(&self) -> bool {
        self.signaled
    }

    /// Set the signaled status to true
    /// * returns `true` for chaining
    pub fn signal(&mut self) -> bool {
        self.signaled = true;
        true
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
