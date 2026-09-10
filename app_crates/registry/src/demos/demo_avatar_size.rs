use dioxus::prelude::*;

use crate::ui::avatar::{Avatar, AvatarFallback, AvatarImage, AvatarSize};

#[component]
pub fn DemoAvatarSize() -> Element {
    rsx! {
        div { class: "flex flex-wrap gap-4 items-center",
            Avatar { size: AvatarSize::Sm,
                AvatarImage { src: "https://api.dicebear.com/9.x/notionists/svg?seed=rustify", alt: "@rustify" }
                AvatarFallback { "RS" }
            }
            Avatar {
                AvatarImage { src: "https://api.dicebear.com/9.x/notionists/svg?seed=rustify", alt: "@rustify" }
                AvatarFallback { "RS" }
            }
            Avatar { size: AvatarSize::Lg,
                AvatarImage { src: "https://api.dicebear.com/9.x/notionists/svg?seed=rustify", alt: "@rustify" }
                AvatarFallback { "RS" }
            }
        }
    }
}
