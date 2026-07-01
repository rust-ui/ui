---
title: "Demo Message Group"
name: "demo_message_group"
cargo_dependencies: []
registry_dependencies: ["avatar", "bubble", "message"]
type: "components:demos"
path: "demos/demo_message_group.rs"
---

# Demo Message Group

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_message_group
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::bubble::{Bubble, BubbleContent, BubbleVariant};
use crate::components::ui::message::{Message, MessageAvatar, MessageContent, MessageGroup};

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
```
