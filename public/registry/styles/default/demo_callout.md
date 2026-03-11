---
title: "Demo Callout"
name: "demo_callout"
cargo_dependencies: []
registry_dependencies: ["callout"]
type: "components:demos"
path: "demos/demo_callout.rs"
---

# Demo Callout

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_callout
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::callout::Callout;

#[component]
pub fn DemoCallout() -> impl IntoView {
    view! { <Callout title="Note">"You can add components and dependencies to your app using the CLI."</Callout> }
}
```
