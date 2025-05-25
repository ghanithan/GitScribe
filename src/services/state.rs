//! Application state management

use dioxus::prelude::*;
use fermi::{Atom, AtomRoot, AtomRef, use_atom_root, use_atom_state};
use std::sync::Arc;

/// App state atom for global state
pub static APP_STATE: Atom<AppState> = Atom::new(AppState::default());

/// Global application state
#[derive(Clone, Debug, Default)]
pub struct AppState {
    /// Whether the application is currently loading
    pub loading: bool,
    /// The current authenticated user, if any
    pub user: Option<User>,
}

/// User information
#[derive(Clone, Debug)]
pub struct User {
    /// The user's name
    pub name: String,
    /// The user's GitHub username
    pub username: String,
    /// The user's avatar URL
    pub avatar_url: String,
}

/// Initialize the application state
pub fn init_state(cx: &ScopeState) {
    use_atom_root(cx);
}

/// Use the application state in components
pub fn use_app_state(cx: &ScopeState) -> AtomRef<AppState> {
    let state = use_atom_state(cx, &APP_STATE);
    state
}

/// Set loading state
pub fn set_loading(cx: &ScopeState, loading: bool) {
    let state = use_app_state(cx);
    let mut app_state = state.get();
    app_state.loading = loading;
    state.set(app_state);
}

/// Set user information
pub fn set_user(cx: &ScopeState, user: Option<User>) {
    let state = use_app_state(cx);
    let mut app_state = state.get();
    app_state.user = user;
    state.set(app_state);
}