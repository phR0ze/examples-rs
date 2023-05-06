//! Provides progress shared state
use std::collections::HashMap;

/// Progress shared state
pub struct ProgressState {
    /// Caching for progress tracking
    progress: HashMap<String, f64>,
}

impl ProgressState {
    /// Get the progress for the given route
    /// * `route: &str` is the route to lookup the progress for
    pub fn get_progress(&self, route: &str) -> f64 {
        *self.progress.get(route).unwrap_or(&0.0)
    }

    /// Set the progress for the given route
    /// * `route: &str` is the route to set the progress for
    /// * `progress: f64` the progress value to set
    pub fn set_current_page(&mut self, route: String, progress: f64) {
        self.progress.insert(route, progress);
    }
}

impl Default for ProgressState {
    fn default() -> Self {
        ProgressState { progress: HashMap::new() }
    }
}
