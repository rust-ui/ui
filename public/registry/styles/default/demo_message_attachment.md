---
title: "Demo Message Attachment"
name: "demo_message_attachment"
cargo_dependencies: ["icons/leptos"]
registry_dependencies: ["attachment", "bubble", "button", "message"]
type: "components:demos"
path: "demos/demo_message_attachment.rs"
---

# Demo Message Attachment

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_message_attachment
```

## Component Code

```rust
use icons::Download;
use leptos::prelude::*;

use crate::components::ui::attachment::{
    Attachment, AttachmentAction, AttachmentActions, AttachmentContent, AttachmentDescription, AttachmentMedia,
    AttachmentMediaVariant, AttachmentOrientation, AttachmentTitle,
};
use crate::components::ui::bubble::{Bubble, BubbleContent, BubbleVariant};
use crate::components::ui::button::{ButtonSize, ButtonVariant};
use crate::components::ui::message::{Message, MessageAlign, MessageContent};

#[component]
pub fn DemoMessageAttachment() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-8 py-12 w-full max-w-sm">
            <Message align=MessageAlign::End>
                <MessageContent>
                    <Attachment orientation=AttachmentOrientation::Vertical>
                        <AttachmentMedia variant=AttachmentMediaVariant::Image>
                            <img
                                src="https://images.unsplash.com/photo-1497366754035-f200968a6e72?w=900&auto=format&fit=crop&q=80"
                                alt="Workspace"
                            />
                        </AttachmentMedia>
                    </Attachment>
                    <Bubble>
                        <BubbleContent>
                            "Here's the image. Can you add it to the PDF? Use it for the cover page."
                        </BubbleContent>
                    </Bubble>
                </MessageContent>
            </Message>
            <Message>
                <MessageContent>
                    <Bubble variant=BubbleVariant::Muted>
                        <BubbleContent>"Done. Here's the PDF with the image added as the cover page."</BubbleContent>
                    </Bubble>
                    <Attachment class="w-full">
                        <AttachmentMedia>
                            <icons::FileText />
                        </AttachmentMedia>
                        <AttachmentContent>
                            <AttachmentTitle>"sales-dashboard.pdf"</AttachmentTitle>
                            <AttachmentDescription>"PDF · 2.4 MB"</AttachmentDescription>
                        </AttachmentContent>
                        <AttachmentActions>
                            <AttachmentAction
                                variant=ButtonVariant::Secondary
                                size=ButtonSize::IconSm
                                attr:title="Download"
                                attr:aria-label="Download"
                            >
                                <Download />
                            </AttachmentAction>
                        </AttachmentActions>
                    </Attachment>
                </MessageContent>
            </Message>
            <Message align=MessageAlign::End>
                <MessageContent>
                    <Bubble>
                        <BubbleContent>"Thanks. Looks good."</BubbleContent>
                    </Bubble>
                </MessageContent>
            </Message>
        </div>
    }
}
```
