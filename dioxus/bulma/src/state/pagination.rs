//! Provides pagination shared state
use std::collections::HashMap;

/// Pagination shared state
pub struct PaginationState {
    /// Caching for current pages tracking
    pub current_pages: HashMap<String, usize>,
}

impl Default for PaginationState {
    fn default() -> Self {
        PaginationState { current_pages: HashMap::new() }
    }
}
