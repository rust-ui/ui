---
title: "Demo Attachment Group"
name: "demo_attachment_group"
cargo_dependencies: ["icons/leptos"]
registry_dependencies: ["attachment"]
type: "components:demos"
path: "demos/demo_attachment_group.rs"
---

# Demo Attachment Group

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_attachment_group
```

## Component Code

```rust
use icons::{FileCode, FileText, Table, X};
use leptos::prelude::*;

use crate::components::ui::attachment::{
    Attachment, AttachmentAction, AttachmentActions, AttachmentContent, AttachmentDescription, AttachmentGroup,
    AttachmentMedia, AttachmentMediaVariant, AttachmentTitle,
};

#[component]
pub fn DemoAttachmentGroup() -> impl IntoView {
    view! {
        <div class="py-12 mx-auto w-full max-w-sm">
            <AttachmentGroup class="w-full">
                <Attachment class="w-64">
                    <AttachmentMedia>
                        <FileText />
                    </AttachmentMedia>
                    <AttachmentContent>
                        <AttachmentTitle>"briefing-notes.pdf"</AttachmentTitle>
                        <AttachmentDescription>"PDF · 1.4 MB"</AttachmentDescription>
                    </AttachmentContent>
                    <AttachmentActions>
                        <AttachmentAction attr:aria-label="Remove briefing-notes.pdf">
                            <X />
                        </AttachmentAction>
                    </AttachmentActions>
                </Attachment>
                <Attachment class="w-64">
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
                    <AttachmentActions>
                        <AttachmentAction attr:aria-label="Remove workspace.png">
                            <X />
                        </AttachmentAction>
                    </AttachmentActions>
                </Attachment>
                <Attachment class="w-64">
                    <AttachmentMedia>
                        <Table />
                    </AttachmentMedia>
                    <AttachmentContent>
                        <AttachmentTitle>"customers.csv"</AttachmentTitle>
                        <AttachmentDescription>"CSV · 18 KB"</AttachmentDescription>
                    </AttachmentContent>
                    <AttachmentActions>
                        <AttachmentAction attr:aria-label="Remove customers.csv">
                            <X />
                        </AttachmentAction>
                    </AttachmentActions>
                </Attachment>
                <Attachment class="w-64">
                    <AttachmentMedia>
                        <FileCode />
                    </AttachmentMedia>
                    <AttachmentContent>
                        <AttachmentTitle>"renderer.tsx"</AttachmentTitle>
                        <AttachmentDescription>"TSX · 12 KB"</AttachmentDescription>
                    </AttachmentContent>
                    <AttachmentActions>
                        <AttachmentAction attr:aria-label="Remove renderer.tsx">
                            <X />
                        </AttachmentAction>
                    </AttachmentActions>
                </Attachment>
            </AttachmentGroup>
        </div>
    }
}
```
