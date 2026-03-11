---
title: "Demo Badge Rtl"
name: "demo_badge_rtl"
cargo_dependencies: []
registry_dependencies: ["badge", "direction_provider"]
type: "components:demos"
path: "demos/demo_badge_rtl.rs"
---

# Demo Badge Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_badge_rtl
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::badge::Badge;
use crate::components::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoBadgeRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <Badge>"افتراضي"</Badge>
        </DirectionProvider>
    }
}
```
