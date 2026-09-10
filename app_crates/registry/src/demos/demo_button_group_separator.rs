use dioxus::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::button_group::{ButtonGroup, ButtonGroupSeparator};

#[component]
pub fn DemoButtonGroupSeparator() -> Element {
    rsx! {
        ButtonGroup {
            Button { variant: ButtonVariant::Secondary, "Copy" }
            ButtonGroupSeparator {}
            Button { variant: ButtonVariant::Secondary, "Paste" }
            ButtonGroupSeparator {}
            Button { variant: ButtonVariant::Secondary, "Cut" }
        }
    }
}
