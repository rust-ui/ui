use dioxus::prelude::*;

use crate::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleGroup, BubbleReactions, BubbleVariant};

#[component]
pub fn DemoBubbleGroup() -> Element {
    rsx! {
        div { class: "flex flex-col gap-8 py-12 w-full max-w-sm",
            Bubble { variant: BubbleVariant::Muted,
                BubbleContent { "Can you tell me what's the issue?" }
            }
            BubbleGroup {
                Bubble { align: BubbleAlign::End,
                    BubbleContent { "You tell me!" }
                }
                Bubble { align: BubbleAlign::End,
                    BubbleContent { "It worked yesterday. You broke it!" }
                }
                Bubble { align: BubbleAlign::End,
                    BubbleContent { "Find the bug and fix it." }
                    BubbleReactions { aria_label: "Reactions: eyes", align: BubbleAlign::Start,
                        span { "👀" }
                    }
                }
            }
            Bubble { variant: BubbleVariant::Muted,
                BubbleContent {
                    "Want me to diff yesterday's you against today's you? It's a bit embarrassing."
                }
            }
        }
    }
}
