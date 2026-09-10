use dioxus::prelude::*;

use crate::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::ui::bubble::{Bubble, BubbleContent, BubbleGroup, BubbleReactions, BubbleVariant};
use crate::ui::message::{Message, MessageAlign, MessageAvatar, MessageContent, MessageFooter};

#[component]
pub fn DemoMessage() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6 py-12 w-full max-w-sm",
            Message { align: MessageAlign::End,
                MessageAvatar {
                    Avatar {
                        AvatarImage { src: "/avatars/10.png", alt: "@me" }
                        AvatarFallback { "ME" }
                    }
                }
                MessageContent {
                    Bubble {
                        BubbleContent { "Deploying to prod real quick." }
                    }
                }
            }
            Message {
                MessageAvatar {
                    Avatar {
                        AvatarImage { src: "/avatars/02.png", alt: "@rabbit" }
                        AvatarFallback { "R" }
                    }
                }
                MessageContent {
                    Bubble { variant: BubbleVariant::Muted,
                        BubbleContent { "It's 4:55 PM. On a Friday." }
                    }
                }
            }
            Message { align: MessageAlign::End,
                MessageAvatar {
                    Avatar {
                        AvatarImage { src: "/avatars/10.png", alt: "@me" }
                        AvatarFallback { "ME" }
                    }
                }
                MessageContent {
                    Bubble {
                        BubbleContent { "It's a one-line change." }
                    }
                    MessageFooter { "Delivered" }
                }
            }
            Message {
                MessageAvatar {
                    Avatar {
                        AvatarImage { src: "/avatars/02.png", alt: "@rabbit" }
                        AvatarFallback { "R" }
                    }
                }
                MessageContent {
                    BubbleGroup {
                        Bubble { variant: BubbleVariant::Muted,
                            BubbleContent { "It's always a one-line change 😭." }
                        }
                        Bubble { variant: BubbleVariant::Muted,
                            BubbleContent { "Alright, let me take a look." }
                            BubbleReactions { aria_label: "Reactions: thumbs up",
                                span { "👍" }
                            }
                        }
                    }
                }
            }
            div {
                role: "status",
                class: "group/marker relative flex min-h-4 w-full items-center gap-2 text-left text-sm text-muted-foreground",
                span { class: "min-w-0 wrap-break-word shimmer",
                    span { class: "font-medium", "Oliver" }
                    " is typing..."
                }
            }
        }
    }
}
