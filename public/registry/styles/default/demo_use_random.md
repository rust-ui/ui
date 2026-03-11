---
title: "Demo Use Random"
name: "demo_use_random"
cargo_dependencies: []
registry_dependencies: []
type: "components:demos"
path: "demos/demo_use_random.rs"
---

# Demo Use Random

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_use_random
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::hooks::use_random::use_random_id;

#[component]
pub fn DemoUseRandom() -> impl IntoView {
    let checkbox_id = use_random_id();

    view! {
        <div class="p-4 mx-auto space-y-4 max-w-md">
            <h3 class="font-semibold">"Random ID Hook"</h3>

            <p class="text-sm">
                "Generated ID: " <code class="py-1 px-2 text-xs rounded bg-muted">{checkbox_id.clone()}</code>
            </p>

            <div class="flex items-center space-x-2">
                <input type="checkbox" id=checkbox_id />
                <label class="text-sm">"Checkbox with unique ID"</label>
            </div>
        </div>
    }
}
```
