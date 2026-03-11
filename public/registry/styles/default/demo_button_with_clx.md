---
title: "Demo Button With Clx"
name: "demo_button_with_clx"
cargo_dependencies: []
registry_dependencies: []
type: "components:demos"
path: "demos/demo_button_with_clx.rs"
---

# Demo Button With Clx

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_button_with_clx
```

## Component Code

```rust
use leptos::prelude::*;
use leptos_ui::clx;

// * 💁 Define your reusable Component here:
clx! {MyButton, button, "px-4 py-2 bg-neutral-900 text-white rounded-md"}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn DemoButtonWithClx() -> impl IntoView {
    let count_signal = RwSignal::new(0);
    let on_click = move |_| *count_signal.write() += 1;

    view! {
        <MyButton class="bg-sky-500" on:click=on_click>
            "Click Me: "
            {count_signal}
        </MyButton>
    }
}
```
