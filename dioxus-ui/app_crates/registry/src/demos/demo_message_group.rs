use dioxus::prelude::*;

use crate::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::ui::bubble::{Bubble, BubbleContent, BubbleVariant};
use crate::ui::message::{Message, MessageAvatar, MessageContent, MessageGroup};

#[component]
pub fn DemoMessageGroup() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6 py-12 w-full max-w-sm",
            MessageGroup {
                Message {
                    MessageAvatar {}
                    MessageContent {
                        Bubble { variant: BubbleVariant::Muted,
                            BubbleContent { "I checked the registry addresses." }
                        }
                    }
                }
                Message {
                    MessageAvatar {
                        Avatar {
                            AvatarImage { src: "/avatars/02.png", alt: "@avatar" }
                            AvatarFallback { "CN" }
                        }
                    }
                    MessageContent {
                        Bubble { variant: BubbleVariant::Muted,
                            BubbleContent { "The component and example JSON now live under the UI registry." }
                        }
                    }
                }
            }
        }
    }
}
