//! Provides some basic shared state
mod pagination;
mod progress;
pub use pagination::*;
pub use progress::*;

// Shared state object
pub struct GlobalState {
    pub progress: ProgressState,
    pub pagination: PaginationState,
}

impl Default for GlobalState {
    fn default() -> Self {
        GlobalState { progress: ProgressState::default(), pagination: PaginationState::default() }
    }
}
