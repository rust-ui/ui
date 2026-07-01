use leptos::prelude::*;

use crate::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleReactions, BubbleVariant};

#[component]
pub fn DemoBubbleVariants() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-12 py-12 w-full max-w-sm">
            <Bubble>
                <BubbleContent>"This is the default primary bubble."</BubbleContent>
            </Bubble>
            <Bubble variant=BubbleVariant::Secondary align=BubbleAlign::End>
                <BubbleContent>"This is the secondary variant."</BubbleContent>
            </Bubble>
            <Bubble variant=BubbleVariant::Muted>
                <BubbleContent>"This one is muted. It uses a lower emphasis color for the chat bubble."</BubbleContent>
                <BubbleReactions attr:role="img" attr:aria-label="Reaction: thumbs up">
                    <span>"👍"</span>
                </BubbleReactions>
            </Bubble>
            <Bubble variant=BubbleVariant::Tinted align=BubbleAlign::End>
                <BubbleContent>
                    "This one is tinted. The tint is a softer color derived from the primary color."
                </BubbleContent>
            </Bubble>
            <Bubble variant=BubbleVariant::Outline>
                <BubbleContent>"We can also use an outlined variant."</BubbleContent>
            </Bubble>
            <Bubble variant=BubbleVariant::Destructive align=BubbleAlign::End>
                <BubbleContent>"Or a destructive variant with a reaction."</BubbleContent>
                <BubbleReactions attr:role="img" attr:aria-label="Reaction: fire">
                    <span>"🔥"</span>
                </BubbleReactions>
            </Bubble>
            <Bubble variant=BubbleVariant::Ghost>
                // TODO PORT: shadcn uses <Markdown> for rich text rendering inside ghost bubble.
                // Leptos has no built-in Markdown component — rendered as plain pre-wrap text.
                <BubbleContent class="whitespace-pre-wrap">
                    "Ghost bubbles work for assistant text, markdown, and other content that should not be framed.\n\nThis is perfect for assistant messages that should not have a frame and can take the full width of the container.\n\nGhost bubbles are full width and can take the full width of the container."
                </BubbleContent>
            </Bubble>
        </div>
    }
}
