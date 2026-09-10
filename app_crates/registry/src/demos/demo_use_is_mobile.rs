use dioxus::prelude::*;
use icons::{Monitor, Smartphone};

use crate::hooks::use_is_mobile::use_is_mobile;

#[component]
pub fn DemoUseIsMobile() -> Element {
    let is_mobile = use_is_mobile();

    rsx! {
        div { class: "flex flex-col gap-3 items-center",
            div { class: "flex gap-2 items-center text-sm font-medium",
                if is_mobile() {
                    Fragment {
                        Smartphone { class: "size-4" }
                        span { "Mobile" }
                    }
                } else {
                    Fragment {
                        Monitor { class: "size-4" }
                        span { "Desktop" }
                    }
                }
            }
            p { class: "text-xs text-muted-foreground", "Resize the window below 768px to toggle." }
        }
    }
}
