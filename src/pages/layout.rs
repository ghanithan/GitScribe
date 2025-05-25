//! Main layout for the application

use dioxus::prelude::*;
use crate::components::Header;

/// Props for the Layout component
#[derive(Props)]
pub struct LayoutProps<'a> {
    /// The content to display in the layout
    children: Element<'a>,
}

/// Main layout component
#[component]
pub fn Layout<'a>(cx: Scope<'a, LayoutProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "min-h-screen bg-gray-50 dark:bg-gray-900 flex flex-col",
            Header {}
            main {
                class: "flex-1 p-6",
                &cx.props.children
            }
            footer {
                class: "bg-white dark:bg-background-dark text-text-light dark:text-text-dark p-4 text-center",
                "GitScribe v0.1.0"
            }
        }
    })
}