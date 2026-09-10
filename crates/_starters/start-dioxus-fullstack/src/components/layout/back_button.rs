use dioxus::prelude::*;
use icons::ChevronLeft;

use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn BackButton() -> Element {
    let navigator = use_navigator();

    rsx! {
        Button {
            variant: ButtonVariant::Ghost,
            size: ButtonSize::Icon,
            onclick: move |_| { navigator.go_back(); },
            ChevronLeft { class: "size-5" }
        }
    }
}
