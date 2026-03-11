---
title: "Demo Status Variants"
name: "demo_status_variants"
cargo_dependencies: []
registry_dependencies: ["status"]
type: "components:demos"
path: "demos/demo_status_variants.rs"
---

# Demo Status Variants

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_status_variants
```

## Component Code

```rust
use leptos::prelude::*;
use leptos_ui::void;

use crate::components::ui::status::{Status, StatusIndactorVariant};

#[component]
pub fn DemoStatusVariants() -> impl IntoView {
    void! {DemoContainer, div, "rounded-md size-16 bg-neutral-500"}

    view! {
        <div class="flex gap-4">
            <Status variant=StatusIndactorVariant::Normal>
                <DemoContainer />
            </Status>
            <Status variant=StatusIndactorVariant::Active>
                <DemoContainer />
            </Status>
            <Status variant=StatusIndactorVariant::Inactive>
                <DemoContainer />
            </Status>
        </div>
    }
}
```
