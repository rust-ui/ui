use dioxus::prelude::*;

use crate::ui::avatar::{Avatar, AvatarBadge, AvatarFallback, AvatarImage};

#[component]
pub fn DemoAvatarBadge() -> Element {
    rsx! {
        div { class: "flex gap-4 items-center",
            Avatar {
                AvatarImage { src: "https://api.dicebear.com/9.x/notionists/svg?seed=alice", alt: "@alice" }
                AvatarFallback { "AL" }
                AvatarBadge { class: "bg-green-500 dark:bg-green-700" }
            }
            Avatar {
                AvatarImage { src: "https://api.dicebear.com/9.x/notionists/svg?seed=bob", alt: "@bob" }
                AvatarFallback { "BO" }
                AvatarBadge { class: "bg-red-500 dark:bg-red-700" }
            }
            Avatar {
                AvatarImage { src: "https://api.dicebear.com/9.x/notionists/svg?seed=carol", alt: "@carol" }
                AvatarFallback { "CA" }
                AvatarBadge {}
            }
        }
    }
}
