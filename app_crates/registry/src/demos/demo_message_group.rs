use leptos::prelude::*;

use crate::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::ui::bubble::{Bubble, BubbleContent, BubbleVariant};
use crate::ui::message::{Message, MessageAvatar, MessageContent, MessageGroup};

#[component]
pub fn DemoMessageGroup() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6 py-12 w-full max-w-sm">
            <MessageGroup>
                <Message>
                    <MessageAvatar />
                    <MessageContent>
                        <Bubble variant=BubbleVariant::Muted>
                            <BubbleContent>"I checked the registry addresses."</BubbleContent>
                        </Bubble>
                    </MessageContent>
                </Message>
                <Message>
                    <MessageAvatar>
                        <Avatar>
                            <AvatarImage attr:src="/avatars/02.png" attr:alt="@avatar" />
                            <AvatarFallback>"CN"</AvatarFallback>
                        </Avatar>
                    </MessageAvatar>
                    <MessageContent>
                        <Bubble variant=BubbleVariant::Muted>
                            <BubbleContent>
                                "The component and example JSON now live under the UI registry."
                            </BubbleContent>
                        </Bubble>
                    </MessageContent>
                </Message>
            </MessageGroup>
        </div>
    }
}
