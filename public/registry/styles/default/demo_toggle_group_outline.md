---
title: "Demo Toggle Group Outline"
name: "demo_toggle_group_outline"
cargo_dependencies: []
registry_dependencies: ["toggle_group"]
type: "components:demos"
path: "demos/demo_toggle_group_outline.rs"
---

# Demo Toggle Group Outline

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_toggle_group_outline
```

## Component Code

```rust
use icons::{Bold, Italic, Underline};
use leptos::prelude::*;

use crate::components::ui::toggle_group::{ToggleGroup, ToggleGroupItem, ToggleGroupVariant};

#[component]
pub fn DemoToggleGroupOutline() -> impl IntoView {
    let bold = RwSignal::new(false);
    let italic = RwSignal::new(false);
    let underline = RwSignal::new(false);

    view! {
        <ToggleGroup variant=ToggleGroupVariant::Outline>
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
