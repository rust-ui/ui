use dioxus::prelude::*;

use crate::ui::bubble::{Bubble, BubbleContent, BubbleVariant};
use crate::ui::message::{Message, MessageAlign, MessageContent, MessageFooter, MessageHeader};

#[component]
pub fn DemoMessageHeaderFooter() -> Element {
    rsx! {
        div { class: "flex flex-col gap-8 py-12 w-full max-w-sm",
            Message {
                MessageContent {
                    MessageHeader { "Olivia" }
                    Bubble { variant: BubbleVariant::Muted,
                        BubbleContent { "I already checked the logs." }
                    }
                }
            }
            Message { align: MessageAlign::End,
                MessageContent {
                    Bubble {
                        BubbleContent { "Send the report to the team. Ping @shadcn if you need help." }
                    }
                    MessageFooter {
                        div {
                            "Read "
                            span { class: "font-normal", "Yesterday" }
                        }
                    }
                }
            }
        }
    }
}
