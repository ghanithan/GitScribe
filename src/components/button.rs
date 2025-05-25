//! Button component for the application

use dioxus::prelude::*;

/// Props for the Button component
#[derive(Props)]
pub struct ButtonProps<'a> {
    /// The text to display on the button
    #[props(into)]
    pub text: String,
    
    /// The button variant (primary, secondary, etc.)
    #[props(default = "btn-primary".to_string())]
    pub class: String,
    
    /// Optional onclick handler
    #[props(optional)]
    pub onclick: Option<EventHandler<'a, MouseEvent>>,
    
    /// Whether the button is disabled
    #[props(default = false)]
    pub disabled: bool,
}

/// Button component
#[component]
pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element<'a> {
    let base_class = "btn ".to_string();
    let class = format!("{}{}", base_class, cx.props.class);
    
    cx.render(rsx! {
        button {
            class: "{class}",
            disabled: cx.props.disabled,
            onclick: move |event| {
                if let Some(handler) = &cx.props.onclick {
                    handler.call(event);
                }
            },
            {cx.props.text.clone()}
        }
    })
}