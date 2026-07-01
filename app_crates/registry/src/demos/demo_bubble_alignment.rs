use leptos::prelude::*;

use crate::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleVariant};

#[component]
pub fn DemoBubbleAlignment() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-8 py-12 w-full max-w-sm">
            <Bubble variant=BubbleVariant::Muted>
                <BubbleContent>"This bubble is aligned to the start. This is the default alignment."</BubbleContent>
            </Bubble>
            <Bubble align=BubbleAlign::End>
                <BubbleContent>"This bubble is aligned to the end. Use this for user messages."</BubbleContent>
            </Bubble>
        </div>
    }
}
