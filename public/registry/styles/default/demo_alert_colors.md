---
title: "Demo Alert Colors"
name: "demo_alert_colors"
cargo_dependencies: []
registry_dependencies: ["alert"]
type: "components:demos"
path: "demos/demo_alert_colors.rs"
---

# Demo Alert Colors

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_alert_colors
```

## Component Code

```rust
use icons::TriangleAlert;
use leptos::prelude::*;

use crate::components::ui::alert::{Alert, AlertDescription, AlertTitle};

#[component]
pub fn DemoAlertColors() -> impl IntoView {
    view! {
        <Alert class="max-w-md text-amber-900 bg-amber-50 border-amber-200 dark:text-amber-50 dark:border-amber-900 dark:bg-amber-950">
            <TriangleAlert />
            <AlertTitle>"Your subscription will expire in 3 days."</AlertTitle>
            <AlertDescription>
                "Renew now to avoid service interruption or upgrade to a paid plan to continue using the service."
            </AlertDescription>
        </Alert>
    }
}
```
