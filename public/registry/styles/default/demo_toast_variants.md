---
title: "Demo Toast Variants"
name: "demo_toast_variants"
cargo_dependencies: []
registry_dependencies: ["button", "toast_custom"]
type: "components:demos"
path: "demos/demo_toast_variants.rs"
---

# Demo Toast Variants

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_toast_variants
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::toast_custom::toast_wrapper::show_toast;

#[component]
pub fn DemoToastVariants() -> impl IntoView {
    let show_success_toast = move |_| {
        show_toast().success("Success!");
    };

    let show_error_toast = move |_| {
        show_toast().error("Error!");
    };

    let show_warning_toast = move |_| {
        show_toast().warning("Warning!");
    };

    view! {
        <div class="flex gap-4">
            <Button variant=ButtonVariant::Success on:click=show_success_toast>
                "Success"
            </Button>
            <Button variant=ButtonVariant::Destructive on:click=show_error_toast>
                "Error"
            </Button>
            <Button variant=ButtonVariant::Warning on:click=show_warning_toast>
                "Warning"
            </Button>
        </div>
    }
}
```
