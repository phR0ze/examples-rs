mod colors;
mod sizes;
pub use colors::*;
pub use sizes::*;

use dioxus::prelude::*;
use fermi::{use_atom_ref, use_atom_root, AtomRef, Readable, UseAtomRef};

/// Unsubscribe from render events for the given atom and scope
pub fn use_unsubscribed_atom_ref<T: 'static>(cx: &ScopeState, atom: AtomRef<T>) -> &UseAtomRef<T> {
    let state = use_atom_ref(cx, atom);

    // Unsubscribe from render events
    let root = use_atom_root(cx);
    root.unsubscribe(atom.unique_id(), cx.scope_id());

    state
}
