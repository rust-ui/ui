use dioxus::prelude::*;
use icons::Plus;

use crate::ui::avatar::{Avatar, AvatarBadge, AvatarFallback, AvatarImage};

#[component]
pub fn DemoAvatarBadgeIcon() -> Element {
    rsx! {
        Avatar {
            AvatarImage { src: "https://api.dicebear.com/9.x/notionists/svg?seed=rustify", alt: "@rustify" }
            AvatarFallback { "RS" }
            AvatarBadge {
                Plus {}
            }
        }
    }
}
