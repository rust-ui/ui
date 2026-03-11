---
title: "Demo Toggle Group Font Weight"
name: "demo_toggle_group_font_weight"
cargo_dependencies: []
registry_dependencies: ["toggle_group"]
type: "components:demos"
path: "demos/demo_toggle_group_font_weight.rs"
---

# Demo Toggle Group Font Weight

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_toggle_group_font_weight
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::toggle_group::{ToggleGroup, ToggleGroupItem};

const WEIGHTS: &[(&str, &str)] = &[
    ("100", "font-thin"),
    ("200", "font-extralight"),
    ("300", "font-light"),
    ("400", "font-normal"),
    ("500", "font-medium"),
    ("600", "font-semibold"),
    ("700", "font-bold"),
    ("800", "font-extrabold"),
    ("900", "font-black"),
];

#[component]
pub fn DemoToggleGroupFontWeight() -> impl IntoView {
    let selected = RwSignal::new("400");

    view! {
        <div class="flex flex-col gap-4 items-center">
            <ToggleGroup spacing=2>
                {WEIGHTS
                    .iter()
                    .map(|(weight, _class)| {
                        let w = *weight;
                        let font_style = format!("font-weight: {w}");
                        view! {
                            <ToggleGroupItem
                                title=format!("Weight {w}")
                                pressed=Memo::new(move |_| selected.get() == w)
                                on:click=move |_| selected.set(w)
                            >
                                <div class="flex flex-col gap-0.5 items-center py-1">
                                    <span class="text-base leading-none" style=font_style.clone()>
                                        "Aa"
                                    </span>
                                    <span class="leading-none text-[10px] text-muted-foreground">{w}</span>
                                </div>
                            </ToggleGroupItem>
                        }
                    })
                    .collect_view()}
            </ToggleGroup>
            <p class="text-sm text-muted-foreground">
                "Selected: "
                <code class="font-mono text-foreground">
                    {move || { WEIGHTS.iter().find(|(w, _)| *w == selected.get()).map(|(_, c)| *c).unwrap_or("") }}
                </code>
            </p>
        </div>
    }
}
```
