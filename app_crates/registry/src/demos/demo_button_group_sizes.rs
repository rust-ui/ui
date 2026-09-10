use dioxus::prelude::*;

use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::button_group::ButtonGroup;

#[component]
pub fn DemoButtonGroupSizes() -> Element {
    rsx! {
        div { class: "flex flex-col gap-4",
            ButtonGroup {
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Cut" }
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Copy" }
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Paste" }
            }
            ButtonGroup {
                Button { variant: ButtonVariant::Outline, "Cut" }
                Button { variant: ButtonVariant::Outline, "Copy" }
                Button { variant: ButtonVariant::Outline, "Paste" }
            }
            ButtonGroup {
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Lg, "Cut" }
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Lg, "Copy" }
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Lg, "Paste" }
            }
        }
    }
}
