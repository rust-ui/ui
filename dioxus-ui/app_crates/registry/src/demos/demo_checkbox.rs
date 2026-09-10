use dioxus::prelude::*;

use crate::ui::checkbox::Checkbox;

#[component]
pub fn DemoCheckbox() -> Element {
    rsx! {
        Checkbox {}
    }
}
