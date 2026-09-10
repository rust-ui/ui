use dioxus::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::button_group::ButtonGroup;

#[component]
pub fn DemoButtonGroup() -> Element {
    rsx! {
        ButtonGroup {
            Button { variant: ButtonVariant::Outline, "First" }
            Button { variant: ButtonVariant::Outline, "Second" }
            Button { variant: ButtonVariant::Outline, "Third" }
        }
    }
}
