use dioxus::prelude::*;

use crate::ui::avatar::{Avatar, AvatarFallback, AvatarGroup, AvatarImage};

#[component]
pub fn DemoAvatarGroup() -> Element {
    rsx! {
        AvatarGroup {
            Avatar {
                AvatarImage { src: "https://api.dicebear.com/9.x/notionists/svg?seed=alice", alt: "@alice" }
                AvatarFallback { "AL" }
            }
            Avatar {
                AvatarImage { src: "https://api.dicebear.com/9.x/notionists/svg?seed=bob", alt: "@bob" }
                AvatarFallback { "BO" }
            }
            Avatar {
                AvatarImage { src: "https://api.dicebear.com/9.x/notionists/svg?seed=carol", alt: "@carol" }
                AvatarFallback { "CA" }
            }
        }
    }
}
