//! Provides progress shared state
use std::collections::HashMap;

const RESOLUTION: usize = 500;
const MIN_INTERVAL_MS: usize = 50;
const DEFAULT_DURATION_MS: usize = 15000;

#[derive(Copy, Clone)]
struct ProgressMeta {
    // Max progress value
    max: f64,

    // Current progress value
    value: f64,

    // Unix timestamp when progress timer started
    start: i64,

    // Duration of the progress bar in milliseconds if timed
    duration: usize,
}

impl Default for ProgressMeta {
    fn default() -> Self {
        ProgressMeta {
            max: 1.0,
            value: 0.0,
            start: chrono::Local::now().timestamp(),
            duration: DEFAULT_DURATION_MS,
        }
    }
}

/// Progress shared state
pub struct ProgressState {
    /// Caching for progress tracking
    progress: HashMap<String, ProgressMeta>,
}

impl ProgressState {
    /// Create progress for the given id or reset if already exists
    /// * `id: &str` id for creating or resetting progress
    /// * `max: f64` the max to use
    /// * `value: f64` the value to set
    pub fn new(&mut self, id: &str, max: f64, value: f64) {
        self.progress.insert(id.to_string(), ProgressMeta { max, value, ..Default::default() });
    }

    /// Create timed progress for the given id or reset if already exists
    /// * `id: &str` id for creating or resetting progress
    /// * `start: i64` unix timestamp when the progress was started
    /// * `duration: usize` milliseconds to wait before progress is complete
    pub fn timed(&mut self, id: &str, start: i64, duration: usize) {
        self.progress.insert(id.to_string(), ProgressMeta { start, duration, ..Default::default() });
    }

    /// Advance the timed progress bar
    /// * `id: &str` id for looking up progress
    /// * `returns: bool` true if completed
    pub fn advance(&mut self, id: &str) -> bool {
        let mut result = false;
        if let Some(meta) = self.progress.get_mut(id) {
            if meta.value < meta.max {
                let elapsed = chrono::Local::now().timestamp() - meta.start;
                meta.value = elapsed as f64 / meta.duration as f64;
                println!("advance: {}", meta.value);
                if elapsed >= meta.duration as i64 {
                    meta.value = meta.max;
                    result = true;
                }
            }
        }
        result
    }

    /// Complete the progress for the given id
    /// * `id: &str` id for looking up progress
    pub fn complete(&mut self, id: &str) {
        if let Some(meta) = self.progress.get_mut(id) {
            meta.value = meta.max;
        }
    }

    /// Get the progress duration for the given id
    /// * `id: &str` id for looking up progress value
    /// * returns `duration: usize`
    pub fn duration(&self, id: &str) -> usize {
        self.progress.get(id).unwrap_or(&ProgressMeta::default()).duration
    }

    /// Check if the given progress already exists by id
    /// * `id: &str` id for looking up progress
    /// * returns `exists: bool`
    pub fn exists(&self, id: &str) -> bool {
        self.progress.contains_key(id)
    }

    /// Get the progress max and value for the given id
    /// * `id: &str` id for looking up progress max and value
    /// * returns `(max: f64, value: f64)`
    pub fn get(&self, id: &str) -> (f64, f64) {
        if let Some(meta) = self.progress.get(id) {
            (meta.max, meta.value)
        } else {
            let meta = ProgressMeta::default();
            (meta.max, meta.value)
        }
    }

    /// Get the interval for the given progress timer
    /// * `id: &str` id for looking up progress value
    /// * returns `interval: usize` milliseconds to wait before firing
    pub fn interval(&self, id: &str) -> usize {
        (self.duration(id) / RESOLUTION).min(MIN_INTERVAL_MS)
    }

    /// Reset progress
    /// * `id: &str` id for creating or resetting progress
    pub fn reset(&mut self, id: &str) {
        self.timed(id, chrono::Local::now().timestamp(), self.duration(id));
    }

    /// Set the progress for the given id
    /// * `id: &str` id for updating up progress value
    /// * `value: f64` the value to set
    pub fn set(&mut self, id: &str, value: f64) {
        if let Some(meta) = self.progress.get_mut(id) {
            meta.value = value;
        }
    }

    /// Get the progress start timestamp for the given id
    /// * `id: &str` id for looking up progress value
    /// * returns `start: i64`
    pub fn start(&self, id: &str) -> i64 {
        self.progress.get(id).unwrap_or(&ProgressMeta::default()).start
    }

    /// Get the progress value for the given id
    /// * `id: &str` id for looking up progress value
    /// * returns `value: f64`
    pub fn value(&self, id: &str) -> f64 {
        self.progress.get(id).unwrap_or(&ProgressMeta::default()).value
    }
}

impl Default for ProgressState {
    fn default() -> Self {
        ProgressState { progress: HashMap::new() }
    }
}
