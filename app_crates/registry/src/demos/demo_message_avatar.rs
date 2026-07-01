use leptos::prelude::*;

use crate::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::ui::bubble::{Bubble, BubbleContent, BubbleGroup, BubbleVariant};
use crate::ui::message::{Message, MessageAlign, MessageAvatar, MessageContent};

#[component]
pub fn DemoMessageAvatar() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6 py-12 w-full max-w-sm">
            <Message>
                <MessageAvatar>
                    <Avatar>
                        <AvatarImage attr:src="/avatars/03.png" attr:alt="@avatar" />
                        <AvatarFallback>"R"</AvatarFallback>
                    </Avatar>
                </MessageAvatar>
                <MessageContent>
                    <Bubble variant=BubbleVariant::Muted>
                        <BubbleContent>"The build failed during dependency installation."</BubbleContent>
                    </Bubble>
                </MessageContent>
            </Message>
            <Message align=MessageAlign::End>
                <MessageAvatar>
                    <Avatar>
                        <AvatarImage attr:src="/avatars/10.png" attr:alt="@avatar" />
                        <AvatarFallback>"R"</AvatarFallback>
                    </Avatar>
                </MessageAvatar>
                <MessageContent>
                    <Bubble>
                        <BubbleContent>"Can you share the exact error?"</BubbleContent>
                    </Bubble>
                </MessageContent>
            </Message>
            <Message>
                <MessageAvatar>
                    <Avatar>
                        <AvatarImage attr:src="/avatars/03.png" attr:alt="@avatar" />
                        <AvatarFallback>"R"</AvatarFallback>
                    </Avatar>
                </MessageAvatar>
                <MessageContent>
                    <BubbleGroup>
                        <Bubble variant=BubbleVariant::Muted>
                            <BubbleContent>"Here's the error from the logs"</BubbleContent>
                        </Bubble>
                        <Bubble variant=BubbleVariant::Muted>
                            <BubbleContent>
                                "Something went wrong with the build. The libraries are not installed correctly. Try running the build again."
                            </BubbleContent>
                        </Bubble>
                    </BubbleGroup>
                </MessageContent>
            </Message>
        </div>
    }
}
