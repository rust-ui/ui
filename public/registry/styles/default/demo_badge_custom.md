---
title: "Demo Badge Custom"
name: "demo_badge_custom"
cargo_dependencies: []
registry_dependencies: ["badge"]
type: "components:demos"
path: "demos/demo_badge_custom.rs"
---

# Demo Badge Custom

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_badge_custom
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::badge::Badge;

#[component]
pub fn DemoBadgeCustom() -> impl IntoView {
    view! {
        <div class="flex gap-2 items-center">
            <Badge class="bg-sky-500">Cutstom</Badge>
        </div>
    }
}
```
