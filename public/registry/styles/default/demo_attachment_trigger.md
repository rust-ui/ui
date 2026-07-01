---
title: "Demo Attachment Trigger"
name: "demo_attachment_trigger"
cargo_dependencies: ["icons/leptos"]
registry_dependencies: ["attachment", "dialog"]
type: "components:demos"
path: "demos/demo_attachment_trigger.rs"
---

# Demo Attachment Trigger

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_attachment_trigger
```

## Component Code

```rust
use icons::{Copy, FileSearch, X};
use leptos::prelude::*;

use crate::components::ui::attachment::{
    Attachment, AttachmentAction, AttachmentActions, AttachmentContent, AttachmentDescription, AttachmentMedia,
    AttachmentTitle, AttachmentTrigger,
};
use crate::components::ui::dialog::{Dialog, DialogBody, DialogContent, DialogDescription, DialogHeader, DialogTitle};

#[component]
pub fn DemoAttachmentTrigger() -> impl IntoView {
    view! {
        <div class="py-12 mx-auto w-full max-w-sm">
            // TODO PORT: shadcn uses <DialogTrigger render={<AttachmentTrigger aria-label="..." />} />
            // (Base UI render prop / asChild — DialogTrigger renders AS AttachmentTrigger).
            // Leptos has no render prop. AttachmentTrigger instead auto-consumes Dialog context
            // and sets data-dialog-trigger when inside <Dialog>, acting as the trigger directly.
            <div class="p-3 rounded-xl bg-accent">
                <Dialog class="w-full">
                    <Attachment class="w-full">
                        <AttachmentMedia>
                            <FileSearch />
                        </AttachmentMedia>
                        <AttachmentContent>
                            <AttachmentTitle>"research-summary.pdf"</AttachmentTitle>
                            <AttachmentDescription>"Open preview dialog"</AttachmentDescription>
                        </AttachmentContent>
                        <AttachmentActions>
                            <AttachmentAction attr:aria-label="Copy link">
                                <Copy />
                            </AttachmentAction>
                            <AttachmentAction attr:aria-label="Remove research-summary.pdf">
                                <X />
                            </AttachmentAction>
                        </AttachmentActions>
                        <AttachmentTrigger attr:aria-label="Preview research-summary.pdf" />
                    </Attachment>

                    <DialogContent class="sm:max-w-md">
                        <DialogBody>
                            <DialogHeader>
                                <DialogTitle>"research-summary.pdf"</DialogTitle>
                                <DialogDescription>
                                    "The attachment trigger fills the card and opens the dialog, while the actions stay independently clickable above it."
                                </DialogDescription>
                            </DialogHeader>
                        </DialogBody>
                    </DialogContent>
                </Dialog>
            </div>
        </div>
    }
}
```
