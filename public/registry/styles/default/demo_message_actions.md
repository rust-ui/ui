---
title: "Demo Message Actions"
name: "demo_message_actions"
cargo_dependencies: ["icons/leptos"]
registry_dependencies: ["bubble", "button", "message"]
type: "components:demos"
path: "demos/demo_message_actions.rs"
---

# Demo Message Actions

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_message_actions
```

## Component Code

```rust
use icons::{Copy, RefreshCcw, ThumbsDown, ThumbsUp};
use leptos::prelude::*;

use crate::components::ui::bubble::{Bubble, BubbleContent, BubbleVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::message::{Message, MessageAlign, MessageContent, MessageFooter};

#[component]
pub fn DemoMessageActions() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-8 py-12 w-full max-w-sm">
            <Message>
                <MessageContent>
                    <Bubble variant=BubbleVariant::Muted>
                        <BubbleContent>"The install failure is coming from the workspace package."</BubbleContent>
                    </Bubble>
                    <MessageFooter>
                        <Button
                            variant=ButtonVariant::Ghost
                            size=ButtonSize::Icon
                            attr:aria-label="Copy"
                            attr:title="Copy"
                        >
                            <Copy />
                        </Button>
                        <Button
                            variant=ButtonVariant::Ghost
                            size=ButtonSize::Icon
                            attr:aria-label="Like"
                            attr:title="Like"
                        >
                            <ThumbsUp />
                        </Button>
                        <Button
                            variant=ButtonVariant::Ghost
                            size=ButtonSize::Icon
                            attr:aria-label="Dislike"
                            attr:title="Dislike"
                        >
                            <ThumbsDown />
                        </Button>
                    </MessageFooter>
                </MessageContent>
            </Message>
            <Message align=MessageAlign::End>
                <MessageContent>
                    <Bubble>
                        <BubbleContent>"Okay drop me a link. Taking a look..."</BubbleContent>
                    </Bubble>
                    <MessageFooter class="gap-2">
                        <span class="font-normal text-destructive">"Failed to send"</span>
                        <Button
                            variant=ButtonVariant::Ghost
                            size=ButtonSize::IconXs
                            attr:title="Retry"
                            attr:aria-label="Retry"
                        >
                            <RefreshCcw />
                        </Button>
                    </MessageFooter>
                </MessageContent>
            </Message>
        </div>
    }
}
```
