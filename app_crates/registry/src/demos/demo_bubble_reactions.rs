use dioxus::prelude::*;

use crate::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleReactions, BubbleReactionsSide, BubbleVariant};
use crate::ui::button::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn DemoBubbleReactions() -> Element {
    rsx! {
        div { class: "flex flex-col gap-12 py-12 w-full max-w-sm",
            Bubble { variant: BubbleVariant::Muted, align: BubbleAlign::End,
                BubbleContent { "I don't need tests, I know my code works." }
                BubbleReactions {
                    align: BubbleAlign::Start,
                    role: "img",
                    aria_label: "Reactions: thumbs up, surprised",
                    span { "👍" }
                    span { "😮" }
                }
            }
            Bubble { variant: BubbleVariant::Muted,
                BubbleContent { "Bold. Fine I'll add some tests. I'll let you know when they're done." }
                BubbleReactions { role: "img", aria_label: "Reactions: eyes, rocket, and 2 more",
                    span { "👀" }
                    span { "🚀" }
                    span { "+2" }
                }
            }
            Bubble { align: BubbleAlign::End,
                BubbleContent { "Tests passed on the first try. All 142 of them. Looking good!" }
                BubbleReactions {
                    side: BubbleReactionsSide::Top,
                    align: BubbleAlign::Start,
                    role: "img",
                    aria_label: "Reactions: party popper, clapping hands",
                    span { "🎉" }
                    span { "👏" }
                }
            }
            Bubble { variant: BubbleVariant::Destructive,
                BubbleContent { "Are you sure I can run this command?" }
                BubbleReactions {
                    Button {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Sm,
                        onclick: move |_| {
                            let _ = web_sys::window().and_then(|w| w.alert_with_message("Running command...").ok());
                        },
                        "Yes, run it"
                    }
                }
            }
        }
    }
}
