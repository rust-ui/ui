---
title: "Demo Progress Label"
name: "demo_progress_label"
cargo_dependencies: []
registry_dependencies: ["field", "progress"]
type: "components:demos"
path: "demos/demo_progress_label.rs"
---

# Demo Progress Label

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_progress_label
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::field::{Field, FieldLabel};
use crate::components::ui::progress::Progress;

#[component]
pub fn DemoProgressLabel() -> impl IntoView {
    view! {
        <Field class="w-full max-w-sm">
            <FieldLabel html_for="progress-upload">
                <span>"Upload progress"</span>
                <span class="ml-auto">"66%"</span>
            </FieldLabel>
            <Progress value=66.0 />
        </Field>
    }
}
```
