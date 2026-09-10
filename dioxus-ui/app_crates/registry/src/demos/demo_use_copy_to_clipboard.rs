use dioxus::prelude::*;
use icons::{Check, Copy};

use crate::hooks::use_copy_clipboard::use_copy_clipboard;
use crate::ui::button::{Button, ButtonVariant};
use crate::ui::input::Input;

#[component]
pub fn DemoUseCopyToClipboard() -> Element {
    let url = use_signal(|| "https://rust-ui.com/docs/components/input".to_string());
    let (copy_to_clipboard, copied) = use_copy_clipboard(Some(2000));

    rsx! {
        div { class: "flex gap-2 items-center",
            Input {
                class: "flex-1",
                value: url(),
                readonly: true,
            }
            Button {
                variant: ButtonVariant::Outline,
                onclick: move |_| copy_to_clipboard(&url()),
                if copied() {
                    Check { class: "w-4 h-4" }
                } else {
                    Copy { class: "w-4 h-4" }
                }
            }
        }
    }
}
