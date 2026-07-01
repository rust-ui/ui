---
title: "Demo Attachment Sizes"
name: "demo_attachment_sizes"
cargo_dependencies: ["icons/leptos"]
registry_dependencies: ["attachment"]
type: "components:demos"
path: "demos/demo_attachment_sizes.rs"
---

# Demo Attachment Sizes

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_attachment_sizes
```

## Component Code

```rust
use icons::FileText;
use leptos::prelude::*;

use crate::components::ui::attachment::{
    Attachment, AttachmentContent, AttachmentDescription, AttachmentMedia, AttachmentSize, AttachmentTitle,
};

#[component]
pub fn DemoAttachmentSizes() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-3 py-12 mx-auto w-full max-w-sm">
            <div class="p-3 rounded-xl bg-accent">
                <Attachment size=AttachmentSize::Default class="w-full">
                    <AttachmentMedia>
                        <FileText />
                    </AttachmentMedia>
                    <AttachmentContent>
                        <AttachmentTitle>"Default attachment"</AttachmentTitle>
                        <AttachmentDescription>"PDF · 2.4 MB"</AttachmentDescription>
                    </AttachmentContent>
                </Attachment>
            </div>
            <div class="p-3 rounded-xl bg-accent">
                <Attachment size=AttachmentSize::Sm class="w-full">
                    <AttachmentMedia>
                        <FileText />
                    </AttachmentMedia>
                    <AttachmentContent>
                        <AttachmentTitle>"Small attachment"</AttachmentTitle>
                        <AttachmentDescription>"PDF · 2.4 MB"</AttachmentDescription>
                    </AttachmentContent>
                </Attachment>
            </div>
            <div class="p-3 rounded-xl bg-accent">
                <Attachment size=AttachmentSize::Xs class="w-full">
                    <AttachmentMedia>
                        <FileText />
                    </AttachmentMedia>
                    <AttachmentContent>
                        <AttachmentTitle>"Extra small attachment"</AttachmentTitle>
                    </AttachmentContent>
                </Attachment>
            </div>
        </div>
    }
}
```
