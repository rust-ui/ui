use dioxus::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::toast_custom::toast_wrapper::show_toast;

#[component]
pub fn DemoToastVariants() -> Element {
    let show_success_toast = move |_| {
        show_toast().success("Success!");
    };

    let show_error_toast = move |_| {
        show_toast().error("Error!");
    };

    let show_warning_toast = move |_| {
        show_toast().warning("Warning!");
    };

    rsx! {
        div { class: "flex gap-4",
            Button { variant: ButtonVariant::Success, onclick: show_success_toast, "Success" }
            Button { variant: ButtonVariant::Destructive, onclick: show_error_toast, "Error" }
            Button { variant: ButtonVariant::Warning, onclick: show_warning_toast, "Warning" }
        }
    }
}
