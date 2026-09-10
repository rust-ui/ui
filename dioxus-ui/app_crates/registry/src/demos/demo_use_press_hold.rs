use dioxus::prelude::*;
use icons::Trash2;

use crate::ui::button_action::ButtonAction;
use crate::ui::toast_custom::toast_wrapper::show_toast;

#[component]
pub fn DemoUsePressHold() -> Element {
    let on_complete = EventHandler::new(move |_| {
        show_toast().success("Action completed!");
    });

    rsx! {
        ButtonAction { on_complete: on_complete, duration_ms: 2000,
            Trash2 {}
            span { "Hold to Delete" }
        }
    }
}
