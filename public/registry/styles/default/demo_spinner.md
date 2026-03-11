---
title: "Demo Spinner"
name: "demo_spinner"
cargo_dependencies: []
registry_dependencies: ["spinner"]
type: "components:demos"
path: "demos/demo_spinner.rs"
---

# Demo Spinner

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_spinner
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::spinner::{Spinner, SpinnerCircle};

#[component]
pub fn DemoSpinner() -> impl IntoView {
    view! {
        <div class="flex gap-4">
            <Spinner />
            <SpinnerCircle />
        </div>
    }
}
```
