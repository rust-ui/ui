use dioxus::prelude::*;

use crate::ui::switch::Switch;

#[component]
pub fn DemoSwitch() -> Element {
    rsx! {
        Switch {}
    }
}
