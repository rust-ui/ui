use leptos::prelude::*;

use crate::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleGroup, BubbleReactions, BubbleVariant};

#[component]
pub fn DemoBubbleGroup() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-8 py-12 w-full max-w-sm">
            <Bubble variant=BubbleVariant::Muted>
                <BubbleContent>"Can you tell me what's the issue?"</BubbleContent>
            </Bubble>
            <BubbleGroup>
                <Bubble align=BubbleAlign::End>
                    <BubbleContent>"You tell me!"</BubbleContent>
                </Bubble>
                <Bubble align=BubbleAlign::End>
                    <BubbleContent>"It worked yesterday. You broke it!"</BubbleContent>
                </Bubble>
                <Bubble align=BubbleAlign::End>
                    <BubbleContent>"Find the bug and fix it."</BubbleContent>
                    <BubbleReactions attr:aria-label="Reactions: eyes" align=BubbleAlign::Start>
                        <span>"👀"</span>
                    </BubbleReactions>
                </Bubble>
            </BubbleGroup>
            <Bubble variant=BubbleVariant::Muted>
                <BubbleContent>
                    "Want me to diff yesterday's you against today's you? It's a bit embarrassing."
                </BubbleContent>
            </Bubble>
        </div>
    }
}
