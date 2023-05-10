//! Provides progress shared state
use std::collections::HashMap;

#[derive(Copy, Clone)]
struct ProgressMeta {
    // Max progress value
    max: f64,

    // Current progress value
    value: f64,

    // Amount to increment the value when advancing if timed
    increment: f64,

    // Number of milliseconds to wait before advancing the value if timed
    interval: usize,

    // Current state of the timer if timed
    paused: bool,
}

impl Default for ProgressMeta {
    fn default() -> Self {
        ProgressMeta { max: 1.0, value: 0.0, increment: 0.01, interval: 250, paused: false }
    }
}

impl Default for &ProgressMeta {
    fn default() -> Self {
        &ProgressMeta { max: 1.0, value: 0.0, increment: 0.01, interval: 250, paused: false }
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
    /// * `max: f64` the max to use
    /// * `value: f64` the value to set
    /// * `increment: f64` amount to increment the value when advancing if timed
    /// * `interval: usize` milliseconds to wait before advancing the value
    pub fn timed(&mut self, id: &str, max: f64, value: f64, increment: f64, interval: usize) {
        self.progress.insert(id.to_string(), ProgressMeta { max, value, increment, interval, paused: false });
    }

    /// Advance the progress bar using the set increment if not paused
    /// * `id: &str` id for creating or resetting progress
    pub fn advance(&mut self, id: &str) {
        if let Some(meta) = self.progress.get_mut(id) {
            if !meta.paused {
                if meta.value + meta.increment <= meta.max {
                    meta.value += meta.increment;
                } else {
                    meta.value = meta.max;
                }
            }
        }
    }

    /// Complete the progress for the given id
    /// * `id: &str` id for looking up progress
    pub fn complete(&mut self, id: &str) {
        if let Some(meta) = self.progress.get_mut(id) {
            meta.value = meta.max;
        }
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
        let meta = self.progress.get(id).unwrap_or_default();
        (meta.max, meta.value)
    }

    /// Get the progress interval value for the given id
    /// * `id: &str` id for looking up progress value
    /// * returns `interval: usize` milliseconds to wait before advancing
    pub fn interval(&self, id: &str) -> usize {
        self.progress.get(id).unwrap_or_default().interval
    }

    /// Pause timer from advancing the progress value
    /// * `id: &str` id for looking up progress
    pub fn pause(&mut self, id: &str) {
        if let Some(meta) = self.progress.get_mut(id) {
            meta.paused = true;
        }
    }

    /// Reset progress
    /// * `id: &str` id for creating or resetting progress
    pub fn reset(&mut self, id: &str) {
        self.set(id, 0.0);
    }

    /// Resume timer to continue advancing the progress value
    /// * `id: &str` id for looking up progress
    pub fn resume(&mut self, id: &str) {
        if let Some(meta) = self.progress.get_mut(id) {
            meta.paused = false;
        }
    }

    /// Set the progress for the given id
    /// * `id: &str` id for updating up progress value
    /// * `value: f64` the value to set
    pub fn set(&mut self, id: &str, value: f64) {
        if let Some(meta) = self.progress.get_mut(id) {
            meta.value = value;
        } else {
            self.progress.insert(id.to_string(), ProgressMeta { value, ..Default::default() });
        }
    }

    /// Get the progress value for the given id
    /// * `id: &str` id for looking up progress value
    /// * returns `value: f64`
    pub fn value(&self, id: &str) -> f64 {
        self.progress.get(id).unwrap_or_default().value
    }
}

impl Default for ProgressState {
    fn default() -> Self {
        ProgressState { progress: HashMap::new() }
    }
}
