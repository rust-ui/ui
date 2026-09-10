use dioxus::prelude::*;

use crate::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoAvatarRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "flex gap-3 items-center max-w-fit",
            Avatar {
                AvatarImage { src: "https://api.dicebear.com/9.x/notionists/svg?seed=rustify", alt: "@rustify" }
                AvatarFallback { "م ع" }
            }
            span { class: "text-sm font-medium", "محمد علي" }
        }
    }
}
