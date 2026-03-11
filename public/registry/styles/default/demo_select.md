---
title: "Demo Select"
name: "demo_select"
cargo_dependencies: []
registry_dependencies: ["select"]
type: "components:demos"
path: "demos/demo_select.rs"
---

# Demo Select

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_select
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::select::{Select, SelectContent, SelectGroup, SelectOption, SelectTrigger, SelectValue};

const OPTIONS: [&str; 3] = ["Components", "Extensions", "Icons"];

#[component]
pub fn DemoSelect() -> impl IntoView {
    view! {
        <Select>
            <SelectTrigger class="w-[150px]">
                <SelectValue placeholder="Please select" />
            </SelectTrigger>

            <SelectContent>
                <SelectGroup>
                    {OPTIONS
                        .into_iter()
                        .map(|option| {
                            view! { <SelectOption value=option>{option}</SelectOption> }
                        })
                        .collect_view()}
                </SelectGroup>
            </SelectContent>
        </Select>
    }
}
```
