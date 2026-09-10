use dioxus::prelude::*;

use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::kbd::Kbd;

#[component]
pub fn DemoKbdButton() -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-4 items-center",
            Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, class: "pr-2",
                "Accept "
                Kbd { "⏎" }
            }
            Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, class: "pr-2",
                "Cancel "
                Kbd { "Esc" }
            }
        }
    }
}
