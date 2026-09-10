use dioxus::prelude::*;

use crate::ui::sonner::SonnerTrigger;

#[component]
pub fn DemoSonner() -> Element {
    rsx! {
        SonnerTrigger { title: "You got a message", description: "You toasted me!", "Toast Me!" }
    }
}
