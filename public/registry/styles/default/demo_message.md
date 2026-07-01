---
title: "Demo Message"
name: "demo_message"
cargo_dependencies: []
registry_dependencies: ["avatar", "bubble", "marker", "message"]
type: "components:demos"
path: "demos/demo_message.rs"
---

# Demo Message

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_message
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::bubble::{Bubble, BubbleContent, BubbleGroup, BubbleReactions, BubbleVariant};
use crate::components::ui::marker::{Marker, MarkerContent};
use crate::components::ui::message::{Message, MessageAlign, MessageAvatar, MessageContent, MessageFooter};

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
```
