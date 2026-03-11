---
title: "Demo Use Copy To Clipboard"
name: "demo_use_copy_to_clipboard"
cargo_dependencies: []
registry_dependencies: ["button", "input"]
type: "components:demos"
path: "demos/demo_use_copy_to_clipboard.rs"
---

# Demo Use Copy To Clipboard

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_use_copy_to_clipboard
```

## Component Code

```rust
use icons::{Check, Copy};
use leptos::prelude::*;

use crate::components::hooks::use_copy_clipboard::use_copy_clipboard;
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::input::Input;

#[component]
pub fn DemoUseCopyToClipboard() -> impl IntoView {
    let url_signal = RwSignal::new("https://rust-ui.com/docs/components/input");
    let (copy_to_clipboard, copied_signal) = use_copy_clipboard(Some(2000));

    let handle_copy = move |_| {
        copy_to_clipboard(url_signal.get());
    };

    view! {
        <div class="flex gap-2">
            <Input prop:value=move || url_signal().to_string() attr:readonly=true class="flex-1" />

            <Button variant=ButtonVariant::Outline on:click=handle_copy>
                {move || {
                    if copied_signal.get() { view! { <Check /> }.into_any() } else { view! { <Copy /> }.into_any() }
                }}
            </Button>
        </div>
    }
}
```
