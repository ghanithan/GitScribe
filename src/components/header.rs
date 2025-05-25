//! Header component for the application

use dioxus::prelude::*;
use crate::utils::{toggle_theme, use_theme, Theme};

/// Header component for the application
#[component]
pub fn Header(cx: Scope) -> Element {
    let theme = use_theme(cx);
    let is_dark = theme.read().current == Theme::Dark;
    
    cx.render(rsx! {
        header {
            class: "bg-white dark:bg-background-dark text-text-light dark:text-text-dark p-4 shadow flex justify-between items-center",
            div {
                class: "text-xl font-bold",
                "GitScribe"
            }
            div {
                class: "flex items-center gap-4",
                button {
                    class: "p-2 rounded-full hover:bg-gray-200 dark:hover:bg-gray-700",
                    onclick: move |_| toggle_theme(cx),
                    svg {
                        class: "w-5 h-5",
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke: "currentColor",
                        if is_dark {
                            // Sun icon for dark mode
                            dioxus::html::div {
                                dangerous_inner_html: r#"<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"></path>"#
                            }
                        } else {
                            // Moon icon for light mode
                            dioxus::html::div {
                                dangerous_inner_html: r#"<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"></path>"#
                            }
                        }
                    }
                }
            }
        }
    })
}