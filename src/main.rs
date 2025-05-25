//! GitScribe - A desktop application for managing static websites on GitHub
//! 
//! This is the main entry point for the application.

use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

mod components;
mod pages;
mod services;
mod utils;

use pages::{HomePage, Layout};
use services::init_state;
use utils::ThemeProvider;

fn main() {
    // Initialize the logger
    env_logger::init();
    
    // Launch the Dioxus desktop application
    dioxus_desktop::launch_cfg(
        App,
        Config::new()
            .with_window(
                WindowBuilder::new()
                    .with_title("GitScribe")
                    .with_inner_size(
                        dioxus_desktop::wry::application::dpi::LogicalSize::new(
                            1280.0, 800.0,
                        ),
                    )
            )
    );
}

/// Root application component
fn App(cx: Scope) -> Element {
    // Initialize application state
    init_state(cx);
    
    cx.render(rsx! {
        ThemeProvider {
            Layout {
                HomePage {}
            }
        }
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
