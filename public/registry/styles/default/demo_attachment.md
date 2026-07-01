---
title: "Demo Attachment"
name: "demo_attachment"
cargo_dependencies: ["icons/leptos"]
registry_dependencies: ["attachment", "spinner"]
type: "components:demos"
path: "demos/demo_attachment.rs"
---

# Demo Attachment

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_attachment
```

## Component Code

```rust
use icons::{FileCode, X};
use leptos::prelude::*;

use crate::components::ui::attachment::{
    Attachment, AttachmentAction, AttachmentActions, AttachmentContent, AttachmentDescription, AttachmentGroup,
    AttachmentMedia, AttachmentMediaVariant, AttachmentState, AttachmentTitle,
};
use crate::components::ui::spinner::Spinner;

#[component]
pub fn DemoAttachment() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-3 py-12 mx-auto w-full max-w-sm">
            <div class="p-3 rounded-xl bg-accent">
                <AttachmentGroup>
                    <Attachment orientation=crate::ui::attachment::AttachmentOrientation::Vertical>
                        <AttachmentMedia variant=AttachmentMediaVariant::Image>
                            <img
                                src="https://images.unsplash.com/photo-1497366754035-f200968a6e72?w=900&auto=format&fit=crop&q=80"
                                alt="Workspace"
                            />
                        </AttachmentMedia>
                        <AttachmentContent>
                            <AttachmentTitle>"workspace.png"</AttachmentTitle>
                            <AttachmentDescription>"PNG · 820 KB"</AttachmentDescription>
                        </AttachmentContent>
                    </Attachment>
                    <Attachment orientation=crate::ui::attachment::AttachmentOrientation::Vertical>
                        <AttachmentMedia variant=AttachmentMediaVariant::Image>
                            <img
                                src="https://images.unsplash.com/photo-1497215728101-856f4ea42174?w=900&auto=format&fit=crop&q=80"
                                alt="Desk"
                            />
                        </AttachmentMedia>
                        <AttachmentContent>
                            <AttachmentTitle>"desk-reference.jpg"</AttachmentTitle>
                            <AttachmentDescription>"JPG · 1.1 MB"</AttachmentDescription>
                        </AttachmentContent>
                    </Attachment>
                    <Attachment orientation=crate::ui::attachment::AttachmentOrientation::Vertical>
                        <AttachmentMedia variant=AttachmentMediaVariant::Image>
                            <img
                                src="https://images.unsplash.com/photo-1497366811353-6870744d04b2?w=900&auto=format&fit=crop&q=80"
                                alt="Office"
                            />
                        </AttachmentMedia>
                        <AttachmentContent>
                            <AttachmentTitle>"office-reference.jpg"</AttachmentTitle>
                            <AttachmentDescription>"JPG · 940 KB"</AttachmentDescription>
                        </AttachmentContent>
                    </Attachment>
                </AttachmentGroup>
            </div>
            <div class="p-3 rounded-xl bg-accent">
                <Attachment state=AttachmentState::Uploading class="w-full">
                    <AttachmentMedia>
                        <Spinner />
                    </AttachmentMedia>
                    <AttachmentContent>
                        <AttachmentTitle>"sales-dashboard.pdf"</AttachmentTitle>
                        <AttachmentDescription>"Uploading · 64%"</AttachmentDescription>
                    </AttachmentContent>
                    <AttachmentActions>
                        <AttachmentAction attr:aria-label="Cancel upload">
                            <X />
                        </AttachmentAction>
                    </AttachmentActions>
                </Attachment>
            </div>
            <div class="p-3 rounded-xl bg-accent">
                <Attachment class="w-full">
                    <AttachmentMedia>
                        <FileCode />
                    </AttachmentMedia>
                    <AttachmentContent>
                        <AttachmentTitle>"message-renderer.tsx"</AttachmentTitle>
                        <AttachmentDescription>"TypeScript · 12 KB"</AttachmentDescription>
                    </AttachmentContent>
                    <AttachmentActions>
                        <AttachmentAction attr:aria-label="Remove message-renderer.tsx">
                            <X />
                        </AttachmentAction>
                    </AttachmentActions>
                </Attachment>
            </div>
        </div>
    }
}
```
