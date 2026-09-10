use dioxus::prelude::*;

use crate::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::ui::bubble::{Bubble, BubbleContent, BubbleGroup, BubbleVariant};
use crate::ui::message::{Message, MessageAlign, MessageAvatar, MessageContent};

#[component]
pub fn DemoMessageAvatar() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6 py-12 w-full max-w-sm",
            Message {
                MessageAvatar {
                    Avatar {
                        AvatarImage { src: "/avatars/03.png", alt: "@avatar" }
                        AvatarFallback { "R" }
                    }
                }
                MessageContent {
                    Bubble { variant: BubbleVariant::Muted,
                        BubbleContent { "The build failed during dependency installation." }
                    }
                }
            }
            Message { align: MessageAlign::End,
                MessageAvatar {
                    Avatar {
                        AvatarImage { src: "/avatars/10.png", alt: "@avatar" }
                        AvatarFallback { "R" }
                    }
                }
                MessageContent {
                    Bubble {
                        BubbleContent { "Can you share the exact error?" }
                    }
                }
            }
            Message {
                MessageAvatar {
                    Avatar {
                        AvatarImage { src: "/avatars/03.png", alt: "@avatar" }
                        AvatarFallback { "R" }
                    }
                }
                MessageContent {
                    BubbleGroup {
                        Bubble { variant: BubbleVariant::Muted,
                            BubbleContent { "Here's the error from the logs" }
                        }
                        Bubble { variant: BubbleVariant::Muted,
                            BubbleContent {
                                "Something went wrong with the build. The libraries are not installed correctly. Try running the build again."
                            }
                        }
                    }
                }
            }
        }
    }
}
