//! Provides progress shared state
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub struct ProgressMeta {
    pub max: f64,
    pub value: f64,
}

impl Default for ProgressMeta {
    fn default() -> Self {
        ProgressMeta { max: 1.0, value: 0.0 }
    }
}

impl Default for &ProgressMeta {
    fn default() -> Self {
        &ProgressMeta { max: 1.0, value: 0.0 }
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
        self.progress.insert(id.to_string(), ProgressMeta { max, value });
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

    /// Reset progress
    /// * `id: &str` id for creating or resetting progress
    pub fn reset(&mut self, id: &str) {
        self.set(id, 0.0);
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
