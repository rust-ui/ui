use dioxus::prelude::*;
use icons::{Check, Copy};

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::input::Input;

#[component]
pub fn DemoInputCopy() -> Element {
    let url = "https://dioxuslabs.com/docs/components/input";
    let mut copied = use_signal(|| false);

    rsx! {
        div { class: "flex gap-2",
            Input { placeholder: "{url}", readonly: true, class: "flex-1" }
            Button {
                variant: ButtonVariant::Outline,
                onclick: move |_| {
                    copied.set(true);
                },
                if copied() {
                    Check { class: "w-4 h-4" }
                } else {
                    Copy { class: "w-4 h-4" }
                }
            }
        }
    }
}
