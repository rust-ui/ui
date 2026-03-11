---
title: "Demo Multi Select Align"
name: "demo_multi_select_align"
cargo_dependencies: []
registry_dependencies: ["multi_select"]
type: "components:demos"
path: "demos/demo_multi_select_align.rs"
---

# Demo Multi Select Align

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_multi_select_align
```

## Component Code

```rust
use std::collections::HashSet;

use leptos::prelude::*;

use crate::components::ui::multi_select::{
    MultiSelect, MultiSelectAlign, MultiSelectContent, MultiSelectGroup, MultiSelectItem, MultiSelectOption,
    MultiSelectTrigger, MultiSelectValue,
};

const FRUITS: [&str; 5] = ["Apple", "Banana", "Orange", "Strawberry", "Mango"];

#[component]
pub fn DemoMultiSelectAlign() -> impl IntoView {
    let start_signal = RwSignal::new(HashSet::<String>::new());
    let end_signal = RwSignal::new(HashSet::<String>::new());

    view! {
        <div class="flex justify-between mx-auto w-full max-w-[400px]">
            // Start alignment
            <div class="flex flex-col gap-2">
                <span class="text-sm text-muted-foreground">"Start"</span>
                <MultiSelect values=start_signal align=MultiSelectAlign::Start>
                    <MultiSelectTrigger class="w-fit">
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

            // End alignment
            <div class="flex flex-col gap-2 items-end">
                <span class="text-sm text-muted-foreground">"End"</span>
                <MultiSelect values=end_signal align=MultiSelectAlign::End>
                    <MultiSelectTrigger class="w-fit">
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
