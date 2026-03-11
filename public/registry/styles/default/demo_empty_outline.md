---
title: "Demo Empty Outline"
name: "demo_empty_outline"
cargo_dependencies: []
registry_dependencies: ["button", "empty"]
type: "components:demos"
path: "demos/demo_empty_outline.rs"
---

# Demo Empty Outline

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_empty_outline
```

## Component Code

```rust
use icons::{Cloud, Upload};
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::empty::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle};

#[component]
pub fn DemoEmptyOutline() -> impl IntoView {
    view! {
        <Empty>
            <EmptyHeader>
                <EmptyMedia variant=EmptyMediaVariant::Icon>
                    <Cloud />
                </EmptyMedia>
                <EmptyTitle>"Cloud Storage Empty"</EmptyTitle>
                <EmptyDescription>"Upload files to your cloud storage to access them anywhere."</EmptyDescription>
            </EmptyHeader>
            <EmptyContent>
                <Button variant=ButtonVariant::Outline size=ButtonSize::Sm>
                    <Upload />
                    "Upload Files"
                </Button>
            </EmptyContent>
        </Empty>
    }
}
```
