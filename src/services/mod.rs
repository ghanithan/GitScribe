//! Service modules for the application

pub mod state;

pub use state::{AppState, User, init_state, use_app_state, set_loading, set_user};