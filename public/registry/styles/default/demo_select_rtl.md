---
title: "Demo Select Rtl"
name: "demo_select_rtl"
cargo_dependencies: []
registry_dependencies: ["direction_provider", "select"]
type: "components:demos"
path: "demos/demo_select_rtl.rs"
---

# Demo Select Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_select_rtl
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::direction_provider::{Direction, DirectionProvider};
use crate::components::ui::select::{Select, SelectContent, SelectGroup, SelectOption, SelectTrigger, SelectValue};

const OPTIONS: [(&str, &str); 3] = [("components", "المكونات"), ("extensions", "الإضافات"), ("icons", "الأيقونات")];

#[component]
pub fn DemoSelectRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <Select>
                <SelectTrigger class="w-[180px]">
                    <SelectValue placeholder="اختر خياراً" />
                </SelectTrigger>

                <SelectContent>
                    <SelectGroup>
                        {OPTIONS
                            .into_iter()
                            .map(|(value, label)| {
                                view! { <SelectOption value=value>{label}</SelectOption> }
                            })
                            .collect_view()}
                    </SelectGroup>
                </SelectContent>
            </Select>
        </DirectionProvider>
    }
}
```
