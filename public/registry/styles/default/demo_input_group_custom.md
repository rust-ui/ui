---
title: "Demo Input Group Custom"
name: "demo_input_group_custom"
cargo_dependencies: []
registry_dependencies: ["input_group"]
type: "components:demos"
path: "demos/demo_input_group_custom.rs"
---

# Demo Input Group Custom

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_input_group_custom
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::input_group::{
    InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupButton, InputGroupButtonSize,
};

#[component]
pub fn DemoInputGroupCustom() -> impl IntoView {
    view! {
        <div class="grid gap-6 w-full max-w-sm">
            // Custom element as control — any element with data-slot="input-group-control" integrates with InputGroup
            <InputGroup>
                <textarea
                    data-slot="input-group-control"
                    class="flex-1 py-2.5 px-3 w-full text-sm bg-transparent rounded-md outline-none resize-none min-h-16 placeholder:text-muted-foreground"
                    placeholder="Write a message..."
                    rows="3"
                />
                <InputGroupAddon align=InputGroupAddonAlign::BlockEnd class="border-t">
                    <InputGroupButton
                        size=InputGroupButtonSize::Sm
                        class="ml-auto bg-primary text-primary-foreground hover:bg-primary/90"
                    >
                        "Send"
                    </InputGroupButton>
                </InputGroupAddon>
            </InputGroup>
        </div>
    }
}
```
