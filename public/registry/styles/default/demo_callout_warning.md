---
title: "Demo Callout Warning"
name: "demo_callout_warning"
cargo_dependencies: []
registry_dependencies: ["callout"]
type: "components:demos"
path: "demos/demo_callout_warning.rs"
---

# Demo Callout Warning

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_callout_warning
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::callout::{Callout, CalloutVariant};

#[component]
pub fn DemoCalloutWarning() -> impl IntoView {
    view! {
        <Callout title="Breaking Change" variant=CalloutVariant::Warning>
            "This API changed in v2.0. Update your imports from "
            <code>"leptos_ui::old"</code>
            " to "
            <code>"leptos_ui::ui"</code>
            "."
        </Callout>
    }
}
```
