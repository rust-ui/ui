---
title: "Demo Sonner"
name: "demo_sonner"
cargo_dependencies: []
registry_dependencies: ["sonner"]
type: "components:demos"
path: "demos/demo_sonner.rs"
---

# Demo Sonner

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_sonner
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::sonner::SonnerTrigger;

#[component]
pub fn DemoSonner() -> impl IntoView {
    view! {
        <SonnerTrigger title="You got a message" description="You toasted me!">
            "Toast Me!"
        </SonnerTrigger>
    }
}
```
