//! Provides pagination shared state
use std::collections::HashMap;

/// Pagination shared state
pub struct PaginationState {
    /// Caching for current pages tracking
    current_pages: HashMap<String, usize>,
}

impl PaginationState {
    pub fn get_current_page(&self, route: &str) -> usize {
        let mut value = *self.current_pages.get(route).unwrap_or(&1) as usize;
        if value == 0 {
            value = 1;
        }
        value
    }
    pub fn set_current_page(&mut self, route: &str, page: usize) {
        self.current_pages.insert(route.to_string(), page);
    }
}

impl Default for PaginationState {
    fn default() -> Self {
        PaginationState { current_pages: HashMap::new() }
    }
}
