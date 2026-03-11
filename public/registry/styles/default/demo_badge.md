---
title: "Demo Badge"
name: "demo_badge"
cargo_dependencies: []
registry_dependencies: ["badge"]
type: "components:demos"
path: "demos/demo_badge.rs"
---

# Demo Badge

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_badge
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::badge::Badge;

#[component]
pub fn DemoBadge() -> impl IntoView {
    view! { <Badge>"Default"</Badge> }
}
```
