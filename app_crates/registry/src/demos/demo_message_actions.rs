use dioxus::prelude::*;
use icons::{Copy, RefreshCcw, ThumbsDown, ThumbsUp};

use crate::ui::bubble::{Bubble, BubbleContent, BubbleVariant};
use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::message::{Message, MessageAlign, MessageContent, MessageFooter};

#[component]
pub fn DemoMessageActions() -> Element {
    rsx! {
        div { class: "flex flex-col gap-8 py-12 w-full max-w-sm",
            Message {
                MessageContent {
                    Bubble { variant: BubbleVariant::Muted,
                        BubbleContent { "The install failure is coming from the workspace package." }
                    }
                    MessageFooter {
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Icon,
                            aria_label: "Copy",
                            title: "Copy",
                            Copy {}
                        }
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Icon,
                            aria_label: "Like",
                            title: "Like",
                            ThumbsUp {}
                        }
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Icon,
                            aria_label: "Dislike",
                            title: "Dislike",
                            ThumbsDown {}
                        }
                    }
                }
            }
            Message { align: MessageAlign::End,
                MessageContent {
                    Bubble {
                        BubbleContent { "Okay drop me a link. Taking a look..." }
                    }
                    MessageFooter { class: "gap-2",
                        span { class: "font-normal text-destructive", "Failed to send" }
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::IconXs,
                            title: "Retry",
                            aria_label: "Retry",
                            RefreshCcw {}
                        }
                    }
                }
            }
        }
    }
}
