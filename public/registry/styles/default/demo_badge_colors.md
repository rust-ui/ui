---
title: "Demo Badge Colors"
name: "demo_badge_colors"
cargo_dependencies: []
registry_dependencies: ["badge"]
type: "components:demos"
path: "demos/demo_badge_colors.rs"
---

# Demo Badge Colors

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_badge_colors
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::badge::{Badge, BadgeVariant};

#[component]
pub fn DemoBadgeColors() -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-2">
            <Badge variant=BadgeVariant::Success>"Success"</Badge>
            <Badge variant=BadgeVariant::Warning>"Warning"</Badge>
            <Badge variant=BadgeVariant::Info>"Info"</Badge>
        </div>
    }
}
```
