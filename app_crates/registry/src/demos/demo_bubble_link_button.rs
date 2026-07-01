use leptos::prelude::*;

use crate::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleGroup, BubbleVariant};

#[component]
pub fn DemoBubbleLinkButton() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-8 py-12 w-full max-w-sm">
            <Bubble variant=BubbleVariant::Muted>
                <BubbleContent>"How can I help you today?"</BubbleContent>
            </Bubble>
            <BubbleGroup>
                <Bubble variant=BubbleVariant::Tinted align=BubbleAlign::End>
                    // TODO PORT: shadcn uses BubbleContent render={<button onClick={() => toast(...)} />}
                    // (asChild/render pattern). Ported as on_click with window.alert fallback.
                    <BubbleContent on_click=Callback::new(|_| {
                        let _ = web_sys::window()
                            .and_then(|w| w.alert_with_message("You clicked forgot password").ok());
                    })>"I forgot my password"</BubbleContent>
                </Bubble>
                <Bubble variant=BubbleVariant::Tinted align=BubbleAlign::End>
                    <BubbleContent on_click=Callback::new(|_| {
                        let _ = web_sys::window()
                            .and_then(|w| w.alert_with_message("You clicked help with subscription").ok());
                    })>"I need help with my subscription"</BubbleContent>
                </Bubble>
                <Bubble variant=BubbleVariant::Tinted align=BubbleAlign::End>
                    <BubbleContent on_click=Callback::new(|_| {
                        let _ = web_sys::window()
                            .and_then(|w| w.alert_with_message("You clicked something else. Talk to a human.").ok());
                    })>"Something else. Talk to a human."</BubbleContent>
                </Bubble>
            </BubbleGroup>
        </div>
    }
}
