---
title: "Demo Message Header Footer"
name: "demo_message_header_footer"
cargo_dependencies: []
registry_dependencies: ["bubble", "message"]
type: "components:demos"
path: "demos/demo_message_header_footer.rs"
---

# Demo Message Header Footer

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_message_header_footer
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::bubble::{Bubble, BubbleContent, BubbleVariant};
use crate::components::ui::message::{Message, MessageAlign, MessageContent, MessageFooter, MessageHeader};

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
```
