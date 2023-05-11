//! Provides progress shared state
use instant::Instant;
use std::collections::HashMap;

const RESOLUTION: u64 = 500;
const MIN_INTERVAL_MS: u64 = 50;
const DEFAULT_DURATION_MS: u64 = 15000;

#[derive(Copy, Clone)]
struct ProgressMeta {
    // Max progress value
    max: f64,

    // Current progress value
    value: f64,

    // Instant the progress timer started
    start: Instant,

    // Duration of the progress bar in milliseconds if timed
    duration: u64,
}

impl Default for ProgressMeta {
    fn default() -> Self {
        ProgressMeta { max: 1.0, value: 0.0, start: Instant::now(), duration: DEFAULT_DURATION_MS }
    }
}

/// Progress shared state
pub struct ProgressState {
    /// Caching for progress tracking
    progress: HashMap<String, ProgressMeta>,
}

impl ProgressState {
    /// Create progress for the given id or reset if already exists and is completed
    /// * `id: &str` id for creating or resetting progress
    /// * `max: f64` the max to use
    /// * `value: f64` the value to set
    pub fn new(&mut self, id: &str, max: f64, value: f64) {
        if !self.exists(id) || self.completed(id) {
            self.progress.insert(id.to_string(), ProgressMeta { max, value, ..Default::default() });
        }
    }

    /// Create timed progress for the given id or reset if already exists and is completed
    /// * `id: &str` id for creating or resetting progress
    /// * `start: Instant` start time for the progress timer
    /// * `duration: u64` milliseconds to wait before progress is complete
    pub fn timed(&mut self, id: &str, start: Instant, duration: u64) {
        if !self.exists(id) || self.completed(id) {
            self.progress.insert(id.to_string(), ProgressMeta { start, duration, ..Default::default() });
        }
    }

    /// Advance the timed progress bar
    /// * `id: &str` id for looking up progress
    /// * `returns: bool` true if completed
    pub fn advance(&mut self, id: &str) -> bool {
        let mut result = false;
        if let Some(meta) = self.progress.get_mut(id) {
            if meta.value < meta.max {
                let elapsed = meta.start.elapsed().as_millis() as u64;
                meta.value = elapsed as f64 / meta.duration as f64;
                if elapsed >= meta.duration {
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

    /// Check if the progress bar is completed
    /// * `id: &str` id for looking up progress
    /// * returns `completed: bool` true if completed
    pub fn completed(&self, id: &str) -> bool {
        let mut result = false;
        if let Some(meta) = self.progress.get(id) {
            result = meta.value >= meta.max;
        }
        result
    }

    /// Get the progress duration for the given id
    /// * `id: &str` id for looking up progress value
    /// * returns `duration: u64`
    pub fn duration(&self, id: &str) -> u64 {
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
    /// * returns `interval: u64` milliseconds to wait before firing
    pub fn interval(&self, id: &str) -> u64 {
        (self.duration(id) / RESOLUTION).min(MIN_INTERVAL_MS)
    }

    /// Remove the indicated progress
    /// * `id: &str` id for lookup up progress
    pub fn remove(&mut self, id: &str) {
        self.progress.remove(id);
    }

    /// Reset progress
    /// * `id: &str` id for creating or resetting progress
    pub fn reset(&mut self, id: &str) {
        self.progress.insert(
            id.to_string(),
            ProgressMeta { start: Instant::now(), duration: self.duration(id), ..Default::default() },
        );
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
    /// * returns `start: Instant` the instant the progress timer started
    pub fn start(&self, id: &str) -> Instant {
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
