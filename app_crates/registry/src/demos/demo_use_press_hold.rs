use icons::Trash2;
use leptos::prelude::*;

use crate::ui::button_action::ButtonAction;
use crate::ui::toast_custom::toast_wrapper::show_toast;

#[component]
pub fn DemoUsePressHold() -> impl IntoView {
    let on_complete = Callback::new(move |_| {
        show_toast().success("Action completed!");
    });

    view! {
        <ButtonAction on_complete=on_complete duration_ms=2000>
            <Trash2 />
            <span>"Hold to Delete"</span>
        </ButtonAction>
    }
}
