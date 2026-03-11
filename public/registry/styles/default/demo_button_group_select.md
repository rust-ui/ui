---
title: "Demo Button Group Select"
name: "demo_button_group_select"
cargo_dependencies: []
registry_dependencies: ["button", "button_group", "select"]
type: "components:demos"
path: "demos/demo_button_group_select.rs"
---

# Demo Button Group Select

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_button_group_select
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::button_group::ButtonGroup;
use crate::components::ui::select::{Select, SelectContent, SelectGroup, SelectOption, SelectTrigger, SelectValue};

#[component]
pub fn DemoButtonGroupSelect() -> impl IntoView {
    view! {
        <ButtonGroup>
            <Select default_value="https">
                <SelectTrigger>
                    <SelectValue placeholder="Protocol" />
                </SelectTrigger>
                <SelectContent>
                    <SelectGroup>
                        <SelectOption value="https">"https://"</SelectOption>
                        <SelectOption value="http">"http://"</SelectOption>
                    </SelectGroup>
                </SelectContent>
            </Select>
            <Button variant=ButtonVariant::Outline>"Go"</Button>
        </ButtonGroup>
    }
}
```
