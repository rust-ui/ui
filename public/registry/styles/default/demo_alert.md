---
title: "Demo Alert"
name: "demo_alert"
cargo_dependencies: []
registry_dependencies: ["alert"]
type: "components:demos"
path: "demos/demo_alert.rs"
---

# Demo Alert

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_alert
```

## Component Code

```rust
use icons::Terminal;
use leptos::prelude::*;

use crate::components::ui::alert::{Alert, AlertDescription, AlertTitle};

#[component]
pub fn DemoAlert() -> impl IntoView {
    view! {
        <Alert>
            <Terminal />
            <AlertTitle>"Heads up !"</AlertTitle>
            <AlertDescription>"You can add components to your app using the cli."</AlertDescription>
        </Alert>
    }
}
```
