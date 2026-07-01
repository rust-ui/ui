use leptos::prelude::*;

use crate::ui::bubble::{Bubble, BubbleContent, BubbleVariant};
use crate::ui::message::{Message, MessageAlign, MessageContent, MessageFooter, MessageHeader};

#[component]
pub fn DemoMessageHeaderFooter() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-8 py-12 w-full max-w-sm">
            <Message>
                <MessageContent>
                    <MessageHeader>"Olivia"</MessageHeader>
                    <Bubble variant=BubbleVariant::Muted>
                        <BubbleContent>"I already checked the logs."</BubbleContent>
                    </Bubble>
                </MessageContent>
            </Message>
            <Message align=MessageAlign::End>
                <MessageContent>
                    <Bubble>
                        <BubbleContent>"Send the report to the team. Ping @shadcn if you need help."</BubbleContent>
                    </Bubble>
                    <MessageFooter>
                        <div>"Read " <span class="font-normal">"Yesterday"</span></div>
                    </MessageFooter>
                </MessageContent>
            </Message>
        </div>
    }
}
