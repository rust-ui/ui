use dioxus::prelude::*;

use crate::ui::avatar::{Avatar, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarImage};

#[component]
pub fn DemoAvatarGroupCountIcon() -> Element {
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
            AvatarGroupCount {
                svg {
                    class: "size-4",
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "M5 12h14" }
                    path { d: "M12 5v14" }
                }
            }
        }
    }
}
