//! AutoForm component — stub for Dioxus.
//!
//! The Leptos version uses a `#[derive(AutoForm)]` proc macro that does not exist
//! in Dioxus. This port provides the component shell so demos compile once a
//! form abstraction is added to the registry.

use dioxus::prelude::*;
use tw_merge::tw_merge;

/// Simplified AutoForm component for Dioxus.
/// Renders a `<form>` wrapping any children (typically form fields + a submit button).
#[component]
pub fn AutoForm(
    /// Optional CSS class for the form element
    #[props(into, optional)]
    class: Option<String>,
    /// Optional children (e.g. submit button)
    children: Element,
    /// Optional submit handler
    #[props(optional)]
    on_submit: Option<EventHandler<FormEvent>>,
) -> Element {
    let merged = tw_merge!("flex flex-col gap-4", class.as_deref().unwrap_or(""));

    let handle_submit = move |e: FormEvent| {
        if let Some(handler) = &on_submit {
            handler.call(e);
        }
    };

    rsx! {
        form {
            class: "{merged}",
            "data-name": "AutoForm",
            onsubmit: handle_submit,
            {children}
        }
    }
}
