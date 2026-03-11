---
title: "Demo Badge Variants"
name: "demo_badge_variants"
cargo_dependencies: []
registry_dependencies: ["badge"]
type: "components:demos"
path: "demos/demo_badge_variants.rs"
---

# Demo Badge Variants

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_badge_variants
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::badge::{Badge, BadgeVariant};

#[component]
pub fn DemoBadgeVariants() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 items-center md:flex-row">
            <Badge>Default</Badge>
            <Badge variant=BadgeVariant::Secondary>Secondary</Badge>
            <Badge variant=BadgeVariant::Destructive>Destructive</Badge>
            <Badge variant=BadgeVariant::Outline>Outline</Badge>
        </div>
    }
}
```
