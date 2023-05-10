//! Provides pagination shared state
use std::collections::HashMap;

/// Pagination shared state
pub struct PaginationState {
    /// Caching for current pages tracking
    current_pages: HashMap<String, usize>,
}

impl PaginationState {
    /// Get pagination for the given id
    /// * `id: &str` identifier for the pagination e.g. a page route
    pub fn get(&self, id: &str) -> usize {
        let mut value = *self.current_pages.get(id).unwrap_or(&1) as usize;
        if value == 0 {
            value = 1;
        }
        value
    }

    /// Set pagination for the given id
    /// * `id: &str` identifier for the pagination e.g. a page route
    /// * `page: usize` current page to set
    pub fn set(&mut self, id: &str, page: usize) {
        self.current_pages.insert(id.to_string(), page);
    }
}

impl Default for PaginationState {
    fn default() -> Self {
        PaginationState { current_pages: HashMap::new() }
    }
}
