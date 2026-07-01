---
title: "Demo Attachment Trigger"
name: "demo_attachment_trigger"
cargo_dependencies: ["icons/leptos"]
registry_dependencies: ["attachment"]
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
// TODO PORT: shadcn attachment-trigger demo uses DialogTrigger render={<AttachmentTrigger />}
// to open a Dialog when clicking the attachment card. This pattern requires Base UI's
// useRender/render prop composition (DialogTrigger wrapping AttachmentTrigger).
// Leptos Dialog doesn't support render prop composition. Ported as a plain
// <AttachmentTrigger on_click=...> that shows an alert instead.
// Full Dialog integration requires Leptos Dialog component support.
use icons::{Copy, FileSearch, X};
use leptos::prelude::*;

use crate::components::ui::attachment::{
    Attachment, AttachmentAction, AttachmentActions, AttachmentContent, AttachmentDescription, AttachmentMedia,
    AttachmentTitle, AttachmentTrigger,
};

#[component]
pub fn DemoAttachmentTrigger() -> impl IntoView {
    view! {
        <div class="py-12 mx-auto w-full max-w-sm">
            // TODO PORT: <Dialog> + DialogTrigger render={<AttachmentTrigger />} not portable.
            // Using on_click alert as placeholder for the dialog preview.
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
                <AttachmentTrigger
                    on_click=Callback::new(|_| {
                        let _ = web_sys::window()
                            .and_then(|w| {
                                w.alert_with_message(
                                        "research-summary.pdf\n\nThe attachment trigger fills the card and opens the dialog, while the actions stay independently clickable above it.",
                                    )
                                    .ok()
                            });
                    })
                    attr:aria-label="Preview research-summary.pdf"
                />
            </Attachment>
        </div>
    }
}
```
