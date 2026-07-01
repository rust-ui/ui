use leptos::prelude::*;

use crate::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleReactions, BubbleReactionsSide, BubbleVariant};
use crate::ui::button::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn DemoBubbleReactions() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-12 py-12 w-full max-w-sm">
            <Bubble variant=BubbleVariant::Muted align=BubbleAlign::End>
                <BubbleContent>"I don't need tests, I know my code works."</BubbleContent>
                <BubbleReactions
                    align=BubbleAlign::Start
                    attr:role="img"
                    attr:aria-label="Reactions: thumbs up, surprised"
                >
                    <span>"👍"</span>
                    <span>"😮"</span>
                </BubbleReactions>
            </Bubble>
            <Bubble variant=BubbleVariant::Muted>
                <BubbleContent>"Bold. Fine I'll add some tests. I'll let you know when they're done."</BubbleContent>
                <BubbleReactions attr:role="img" attr:aria-label="Reactions: eyes, rocket, and 2 more">
                    <span>"👀"</span>
                    <span>"🚀"</span>
                    <span>"+2"</span>
                </BubbleReactions>
            </Bubble>
            <Bubble align=BubbleAlign::End>
                <BubbleContent>"Tests passed on the first try. All 142 of them. Looking good!"</BubbleContent>
                <BubbleReactions
                    side=BubbleReactionsSide::Top
                    align=BubbleAlign::Start
                    attr:role="img"
                    attr:aria-label="Reactions: party popper, clapping hands"
                >
                    <span>"🎉"</span>
                    <span>"👏"</span>
                </BubbleReactions>
            </Bubble>
            <Bubble variant=BubbleVariant::Destructive>
                <BubbleContent>"Are you sure I can run this command?"</BubbleContent>
                <BubbleReactions>
                    // TODO PORT: shadcn uses size="xs" (non-icon small). Using ButtonSize::Sm — no Xs variant.
                    <Button
                        variant=ButtonVariant::Ghost
                        size=ButtonSize::Sm
                        on:click=move |_| {
                            let _ = web_sys::window().unwrap().alert_with_message("Running command...");
                        }
                    >
                        "Yes, run it"
                    </Button>
                </BubbleReactions>
            </Bubble>
        </div>
    }
}
