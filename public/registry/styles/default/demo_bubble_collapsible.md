---
title: "Demo Bubble Collapsible"
name: "demo_bubble_collapsible"
cargo_dependencies: ["icons/leptos"]
registry_dependencies: ["bubble", "button", "collapsible"]
type: "components:demos"
path: "demos/demo_bubble_collapsible.rs"
---

# Demo Bubble Collapsible

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_bubble_collapsible
```

## Component Code

```rust
use icons::ChevronDown;
use leptos::prelude::*;

use crate::components::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleVariant};
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::collapsible::{Collapsible, CollapsibleTrigger};

const TEXT: &str = "The accessibility review found two focus states that were visually too subtle in dark mode.\n\nI checked the dialog, menu, and drawer paths because each one renders focusable controls inside a layered surface.\n\nThe dialog and drawer are fine. The menu needs the hover and focus tokens split so keyboard focus stays visible when the pointer is not involved.\n\nI also recommend keeping the change in the style file instead of the primitive so the other themes can choose their own focus treatment later.";

const PREVIEW_LEN: usize = 180;

#[component]
pub fn DemoBubbleCollapsible() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <div class="flex w-full max-w-sm flex-col gap-8 py-12">
            <Bubble variant=BubbleVariant::Muted>
                <BubbleContent>"How can I help you today?"</BubbleContent>
            </Bubble>
            <Bubble variant=BubbleVariant::Muted align=BubbleAlign::End>
                <BubbleContent class="whitespace-pre-line">
                    <Collapsible open=open>
                        <div>
                            {move || if open.get() { TEXT } else { &TEXT[..PREVIEW_LEN] }}
                        </div>
                        // TODO PORT: shadcn CollapsibleTrigger uses render={<Button variant="link" .../>}
                        // (asChild pattern). Ported as CollapsibleTrigger wrapping a Button.
                        <CollapsibleTrigger>
                            <Button
                                variant=ButtonVariant::Link
                                class="gap-1 p-0 text-muted-foreground"
                            >
                                {move || if open.get() { "Show less" } else { "Show more" }}
                                <ChevronDown
                                    attr:data-icon="inline-end"
                                    attr:class=move || {
                                        if open.get() { "rotate-180 transition-transform" } else { "transition-transform" }
                                    }
                                />
                            </Button>
                        </CollapsibleTrigger>
                    </Collapsible>
                </BubbleContent>
            </Bubble>
        </div>
    }
}
```
