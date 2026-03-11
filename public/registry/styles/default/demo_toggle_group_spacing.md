---
title: "Demo Toggle Group Spacing"
name: "demo_toggle_group_spacing"
cargo_dependencies: []
registry_dependencies: ["toggle_group"]
type: "components:demos"
path: "demos/demo_toggle_group_spacing.rs"
---

# Demo Toggle Group Spacing

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_toggle_group_spacing
```

## Component Code

```rust
use icons::{Bold, Italic, Underline};
use leptos::prelude::*;

use crate::components::ui::toggle_group::{ToggleGroup, ToggleGroupItem};

#[component]
pub fn DemoToggleGroupSpacing() -> impl IntoView {
    let bold = RwSignal::new(false);
    let italic = RwSignal::new(false);
    let underline = RwSignal::new(false);

    view! {
        <ToggleGroup spacing=0>
            <ToggleGroupItem title="Bold" pressed=bold on:click=move |_| bold.update(|v| *v = !*v)>
                <Bold />
            </ToggleGroupItem>
            <ToggleGroupItem title="Italic" pressed=italic on:click=move |_| italic.update(|v| *v = !*v)>
                <Italic />
            </ToggleGroupItem>
            <ToggleGroupItem title="Underline" pressed=underline on:click=move |_| underline.update(|v| *v = !*v)>
                <Underline />
            </ToggleGroupItem>
        </ToggleGroup>
    }
}
```
