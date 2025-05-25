//! Common theme provider for the application

use dioxus::prelude::*;

/// Theme options for the application
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Theme {
    Light,
    Dark,
}

/// Global state for theming
#[derive(Clone, Debug)]
pub struct ThemeState {
    pub current: Theme,
}

impl Default for ThemeState {
    fn default() -> Self {
        Self {
            current: Theme::Light,
        }
    }
}

/// Theme provider component
#[component]
pub fn ThemeProvider(cx: Scope, children: Element) -> Element {
    let theme = use_ref(cx, ThemeState::default);
    
    cx.render(rsx! {
        div {
            class: if theme.read().current == Theme::Dark { "dark" } else { "" },
            children
        }
    })
}

/// Hook to use theme in components
pub fn use_theme(cx: &ScopeState) -> &UseRef<ThemeState> {
    use_context_provider(cx, || UseRef::new(ThemeState::default()))
}

/// Toggle theme function for components
pub fn toggle_theme(cx: &ScopeState) {
    let theme = use_theme(cx);
    let mut theme = theme.write();
    theme.current = match theme.current {
        Theme::Light => Theme::Dark,
        Theme::Dark => Theme::Light,
    };
}