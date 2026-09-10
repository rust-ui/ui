use dioxus::prelude::*;

use crate::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleReactions, BubbleVariant};

#[component]
pub fn DemoBubbleVariants() -> Element {
    rsx! {
        div { class: "flex flex-col gap-12 py-12 w-full max-w-sm",
            Bubble {
                BubbleContent { "This is the default primary bubble." }
            }
            Bubble { variant: BubbleVariant::Secondary, align: BubbleAlign::End,
                BubbleContent { "This is the secondary variant." }
            }
            Bubble { variant: BubbleVariant::Muted,
                BubbleContent { "This one is muted. It uses a lower emphasis color for the chat bubble." }
                BubbleReactions { role: "img", aria_label: "Reaction: thumbs up",
                    span { "👍" }
                }
            }
            Bubble { variant: BubbleVariant::Tinted, align: BubbleAlign::End,
                BubbleContent {
                    "This one is tinted. The tint is a softer color derived from the primary color."
                }
            }
            Bubble { variant: BubbleVariant::Outline,
                BubbleContent { "We can also use an outlined variant." }
            }
            Bubble { variant: BubbleVariant::Destructive, align: BubbleAlign::End,
                BubbleContent { "Or a destructive variant with a reaction." }
                BubbleReactions { role: "img", aria_label: "Reaction: fire",
                    span { "🔥" }
                }
            }
            Bubble { variant: BubbleVariant::Ghost,
                BubbleContent { class: "whitespace-pre-wrap",
                    "Ghost bubbles work for assistant text, markdown, and other content that should not be framed.\n\nThis is perfect for assistant messages that should not have a frame and can take the full width of the container.\n\nGhost bubbles are full width and can take the full width of the container."
                }
            }
        }
    }
}
