---
title: "Demo Button Action"
name: "demo_button_action"
cargo_dependencies: []
registry_dependencies: ["button_action", "toast_custom"]
type: "components:demos"
path: "demos/demo_button_action.rs"
---

# Demo Button Action

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_button_action
```

## Component Code

```rust
use icons::Trash2;
use leptos::prelude::*;

use crate::components::ui::button_action::ButtonAction;
use crate::components::ui::toast_custom::toast_wrapper::show_toast;

#[component]
pub fn DemoButtonAction() -> impl IntoView {
    let on_complete = Callback::new(move |_| {
        show_toast().success("Action completed!");
    });

    view! {
        <ButtonAction on_complete=on_complete duration_ms=2000>
            <Trash2 />
            <span>"Hold to Delete"</span>
        </ButtonAction>
    }
}
```
