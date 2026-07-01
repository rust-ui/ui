use leptos::prelude::*;

use crate::ui::bubble::{Bubble, BubbleContent, BubbleGroup, BubbleReactions, BubbleVariant};

#[component]
pub fn DemoBubble() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-8 py-12 w-full max-w-sm">
            <Bubble align=crate::ui::bubble::BubbleAlign::End>
                <BubbleContent>"Hey there! what's up?"</BubbleContent>
            </Bubble>
            <BubbleGroup>
                <Bubble variant=BubbleVariant::Muted>
                    <BubbleContent>"Hey! Want to see chat bubbles?"</BubbleContent>
                </Bubble>
                <Bubble variant=BubbleVariant::Muted>
                    <BubbleContent>
                        "I can group messages, switch sides, and keep the whole thread easy to scan."
                    </BubbleContent>
                    <BubbleReactions attr:role="img" attr:aria-label="Reaction: thumbs up">
                        <span>"👍"</span>
                    </BubbleReactions>
                </Bubble>
            </BubbleGroup>
            <Bubble align=crate::ui::bubble::BubbleAlign::End>
                <BubbleContent>"Sure. Hit me with your best demo."</BubbleContent>
            </Bubble>
            <Bubble variant=BubbleVariant::Muted>
                <BubbleContent>
                    "Yes. You are reading a demo that is demoing itself. Very meta. Very on-brand."
                </BubbleContent>
                <BubbleReactions attr:role="img" attr:aria-label="Reactions: thumbs up, fire, eyes, and 2 more">
                    <span>"👍"</span>
                    <span>"🔥"</span>
                    <span>"👀"</span>
                    <span>"+2"</span>
                </BubbleReactions>
            </Bubble>
        </div>
    }
}
