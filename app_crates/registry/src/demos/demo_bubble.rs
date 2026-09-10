use dioxus::prelude::*;

use crate::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleGroup, BubbleReactions, BubbleVariant};

#[component]
pub fn DemoBubble() -> Element {
    rsx! {
        div { class: "flex flex-col gap-8 py-12 w-full max-w-sm",
            Bubble { align: BubbleAlign::End,
                BubbleContent { "Hey there! what's up?" }
            }
            BubbleGroup {
                Bubble { variant: BubbleVariant::Muted,
                    BubbleContent { "Hey! Want to see chat bubbles?" }
                }
                Bubble { variant: BubbleVariant::Muted,
                    BubbleContent {
                        "I can group messages, switch sides, and keep the whole thread easy to scan."
                    }
                    BubbleReactions { role: "img", aria_label: "Reaction: thumbs up",
                        span { "👍" }
                    }
                }
            }
            Bubble { align: BubbleAlign::End,
                BubbleContent { "Sure. Hit me with your best demo." }
            }
            Bubble { variant: BubbleVariant::Muted,
                BubbleContent {
                    "Yes. You are reading a demo that is demoing itself. Very meta. Very on-brand."
                }
                BubbleReactions { role: "img", aria_label: "Reactions: thumbs up, fire, eyes, and 2 more",
                    span { "👍" }
                    span { "🔥" }
                    span { "👀" }
                    span { "+2" }
                }
            }
        }
    }
}
