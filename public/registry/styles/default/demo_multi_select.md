---
title: "Demo Multi Select"
name: "demo_multi_select"
cargo_dependencies: []
registry_dependencies: ["multi_select"]
type: "components:demos"
path: "demos/demo_multi_select.rs"
---

# Demo Multi Select

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_multi_select
```

## Component Code

```rust
use std::collections::HashSet;

use leptos::prelude::*;

use crate::components::ui::multi_select::{
    MultiSelect, MultiSelectContent, MultiSelectGroup, MultiSelectItem, MultiSelectOption, MultiSelectTrigger,
    MultiSelectValue,
};

const FRUITS: [&str; 5] = ["Apple", "Banana", "Orange", "Strawberry", "Mango"];

#[component]
pub fn DemoMultiSelect() -> impl IntoView {
    let fruits_signal = RwSignal::new(HashSet::<String>::new());

    view! {
        <div class="flex flex-col gap-4">
            <span class="text-sm">
                {move || {
                    let values = fruits_signal.get();
                    if values.is_empty() {
                        "No fruits selected".to_string()
                    } else {
                        format!("Selected: {}", values.iter().cloned().collect::<Vec<_>>().join(", "))
                    }
                }}
            </span>

            <div class="mx-auto">
                <MultiSelect values=fruits_signal>
                    <MultiSelectTrigger class="w-[150px]">
                        <MultiSelectValue placeholder="Select fruits" />
                    </MultiSelectTrigger>

                    <MultiSelectContent>
                        <MultiSelectGroup>
                            {FRUITS
                                .into_iter()
                                .map(|fruit| {
                                    view! {
                                        <MultiSelectItem>
                                            <MultiSelectOption value=fruit>{fruit}</MultiSelectOption>
                                        </MultiSelectItem>
                                    }
                                })
                                .collect_view()}
                        </MultiSelectGroup>
                    </MultiSelectContent>
                </MultiSelect>
            </div>
        </div>
    }
}
```
