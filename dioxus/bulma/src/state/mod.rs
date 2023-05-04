//! Provides some basic shared state
mod pagination;
pub use pagination::*;

// Shared state object
pub struct GlobalState {
    pub pagination: PaginationState,
}

impl Default for GlobalState {
    fn default() -> Self {
        GlobalState { pagination: PaginationState::default() }
    }
}
