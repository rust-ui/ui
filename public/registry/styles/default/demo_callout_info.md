---
title: "Demo Callout Info"
name: "demo_callout_info"
cargo_dependencies: []
registry_dependencies: ["callout"]
type: "components:demos"
path: "demos/demo_callout_info.rs"
---

# Demo Callout Info

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_callout_info
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::callout::{Callout, CalloutVariant};

#[component]
pub fn DemoCalloutInfo() -> impl IntoView {
    view! {
        <Callout title="Did you know?" variant=CalloutVariant::Info>
            "Components are server-side rendered by default. Use "
            <code>"#[component]"</code>
            " to define reactive Leptos components."
        </Callout>
    }
}
```
