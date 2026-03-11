---
title: "Demo Toast"
name: "demo_toast"
cargo_dependencies: []
registry_dependencies: ["button", "toast_custom"]
type: "components:demos"
path: "demos/demo_toast.rs"
---

# Demo Toast

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_toast
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::Button;
use crate::components::ui::toast_custom::toast_wrapper::show_toast;

#[component]
pub fn DemoToast() -> impl IntoView {
    let toast_me = move |_| {
        show_toast().info("This is a toast!");
    };

    view! { <Button on:click=toast_me>"Toast me"</Button> }
}
```
