use dioxus::prelude::*;
use icons::Plus;

use crate::ui::avatar::{Avatar, AvatarFallback, AvatarGroup, AvatarImage, AvatarSize};
use crate::ui::button::{Button, ButtonSize};
use crate::ui::empty::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle};

#[component]
pub fn DemoEmptyAvatarGroup() -> Element {
    rsx! {
        Empty {
            EmptyHeader {
                EmptyMedia {
                    AvatarGroup {
                        Avatar { size: AvatarSize::Lg,
                            AvatarImage { src: "https://api.dicebear.com/9.x/notionists/svg?seed=alice", alt: "@alice" }
                            AvatarFallback { "AL" }
                        }
                        Avatar { size: AvatarSize::Lg,
                            AvatarImage { src: "https://api.dicebear.com/9.x/notionists/svg?seed=bob", alt: "@bob" }
                            AvatarFallback { "BO" }
                        }
                        Avatar { size: AvatarSize::Lg,
                            AvatarImage { src: "https://api.dicebear.com/9.x/notionists/svg?seed=carol", alt: "@carol" }
                            AvatarFallback { "CA" }
                        }
                    }
                }
                EmptyTitle { "No Team Members" }
                EmptyDescription { "Invite your team to collaborate on this project." }
            }
            EmptyContent {
                Button { size: ButtonSize::Sm,
                    Plus {}
                    "Invite Members"
                }
            }
        }
    }
}
