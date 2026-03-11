---
title: "Demo Toggle Group Rtl"
name: "demo_toggle_group_rtl"
cargo_dependencies: []
registry_dependencies: ["direction_provider", "toggle_group"]
type: "components:demos"
path: "demos/demo_toggle_group_rtl.rs"
---

# Demo Toggle Group Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_toggle_group_rtl
```

## Component Code

```rust
use icons::{Bold, Italic, Underline};
use leptos::prelude::*;

use crate::components::ui::direction_provider::{Direction, DirectionProvider};
use crate::components::ui::toggle_group::{ToggleGroup, ToggleGroupItem};

#[component]
pub fn DemoToggleGroupRtl() -> impl IntoView {
    let bold = RwSignal::new(false);
    let italic = RwSignal::new(false);
    let underline = RwSignal::new(false);

    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <ToggleGroup>
                <ToggleGroupItem title="غامق" pressed=bold on:click=move |_| bold.update(|v| *v = !*v)>
                    <Bold />
                </ToggleGroupItem>
                <ToggleGroupItem title="مائل" pressed=italic on:click=move |_| italic.update(|v| *v = !*v)>
                    <Italic />
                </ToggleGroupItem>
                <ToggleGroupItem title="تسطير" pressed=underline on:click=move |_| underline.update(|v| *v = !*v)>
                    <Underline />
                </ToggleGroupItem>
            </ToggleGroup>
        </DirectionProvider>
    }
}
```
