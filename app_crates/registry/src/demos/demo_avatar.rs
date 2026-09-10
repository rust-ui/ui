use dioxus::prelude::*;

use crate::ui::avatar::{Avatar, AvatarFallback, AvatarImage};

#[component]
pub fn DemoAvatar() -> Element {
    rsx! {
        Avatar {
            AvatarImage { src: "https://github.com/shadcn.png", alt: "@shadcn" }
            AvatarFallback { "CN" }
        }
    }
}
