use leptos::prelude::*;

use crate::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::ui::bubble::{Bubble, BubbleContent, BubbleGroup, BubbleReactions, BubbleVariant};
use crate::ui::marker::{Marker, MarkerContent};
use crate::ui::message::{Message, MessageAlign, MessageAvatar, MessageContent, MessageFooter};

#[component]
pub fn DemoMessage() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6 py-12 w-full max-w-sm">
            <Message align=MessageAlign::End>
                <MessageAvatar>
                    <Avatar>
                        <AvatarImage attr:src="/avatars/10.png" attr:alt="@me" />
                        <AvatarFallback>"ME"</AvatarFallback>
                    </Avatar>
                </MessageAvatar>
                <MessageContent>
                    <Bubble>
                        <BubbleContent>"Deploying to prod real quick."</BubbleContent>
                    </Bubble>
                </MessageContent>
            </Message>
            <Message>
                <MessageAvatar>
                    <Avatar>
                        <AvatarImage attr:src="/avatars/02.png" attr:alt="@rabbit" />
                        <AvatarFallback>"R"</AvatarFallback>
                    </Avatar>
                </MessageAvatar>
                <MessageContent>
                    <Bubble variant=BubbleVariant::Muted>
                        <BubbleContent>"It's 4:55 PM. On a Friday."</BubbleContent>
                    </Bubble>
                </MessageContent>
            </Message>
            <Message align=MessageAlign::End>
                <MessageAvatar>
                    <Avatar>
                        <AvatarImage attr:src="/avatars/10.png" attr:alt="@me" />
                        <AvatarFallback>"ME"</AvatarFallback>
                    </Avatar>
                </MessageAvatar>
                <MessageContent>
                    <Bubble>
                        <BubbleContent>"It's a one-line change."</BubbleContent>
                    </Bubble>
                    <MessageFooter>"Delivered"</MessageFooter>
                </MessageContent>
            </Message>
            <Message>
                <MessageAvatar>
                    <Avatar>
                        <AvatarImage attr:src="/avatars/02.png" attr:alt="@rabbit" />
                        <AvatarFallback>"R"</AvatarFallback>
                    </Avatar>
                </MessageAvatar>
                <MessageContent>
                    <BubbleGroup>
                        <Bubble variant=BubbleVariant::Muted>
                            <BubbleContent>"It's always a one-line change 😭."</BubbleContent>
                        </Bubble>
                        <Bubble variant=BubbleVariant::Muted>
                            <BubbleContent>"Alright, let me take a look."</BubbleContent>
                            <BubbleReactions attr:aria-label="Reactions: thumbs up">
                                <span>"👍"</span>
                            </BubbleReactions>
                        </Bubble>
                    </BubbleGroup>
                </MessageContent>
            </Message>
            <Marker role="status">
                <MarkerContent class="shimmer">
                    <span class="font-medium">"Oliver"</span>
                    " is typing..."
                </MarkerContent>
            </Marker>
        </div>
    }
}
