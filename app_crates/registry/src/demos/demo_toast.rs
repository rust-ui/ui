use dioxus::prelude::*;

use crate::ui::toast::ToastTrigger;

#[component]
pub fn DemoToast() -> Element {
    rsx! {
        ToastTrigger { title: "You got a message", description: "You toasted me!", "Toast Me!" }
    }
}
