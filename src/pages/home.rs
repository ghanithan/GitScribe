//! Home page for the application

use dioxus::prelude::*;
use crate::components::Button;

/// Home page component
#[component]
pub fn HomePage(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "max-w-4xl mx-auto",
            h1 {
                class: "text-3xl font-bold text-text-light dark:text-text-dark mb-6",
                "Welcome to GitScribe"
            }
            p {
                class: "text-gray-600 dark:text-gray-300 mb-6",
                "Create, manage, and publish static websites directly from your GitHub repositories."
            }
            div {
                class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                div {
                    class: "card",
                    h2 {
                        class: "text-xl font-bold mb-4 text-text-light dark:text-text-dark",
                        "Create New Project"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "Start a new static website project with GitScribe."
                    }
                    Button {
                        text: "New Project",
                        class: "btn-primary"
                    }
                }
                div {
                    class: "card",
                    h2 {
                        class: "text-xl font-bold mb-4 text-text-light dark:text-text-dark",
                        "Open Existing Project"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "Continue working on an existing GitHub repository."
                    }
                    Button {
                        text: "Open Project",
                        class: "btn-secondary"
                    }
                }
            }
        }
    })
}