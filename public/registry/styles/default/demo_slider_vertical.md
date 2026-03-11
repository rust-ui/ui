---
title: "Demo Slider Vertical"
name: "demo_slider_vertical"
cargo_dependencies: []
registry_dependencies: ["slider"]
type: "components:demos"
path: "demos/demo_slider_vertical.rs"
---

# Demo Slider Vertical

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_slider_vertical
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::slider::{Slider, SliderVariant};

#[component]
pub fn DemoSliderVertical() -> impl IntoView {
    view! {
        <div class="flex gap-10 items-end">
            <div class="flex flex-col gap-3 items-center">
                <div class="flex justify-center items-center h-40">
                    <Slider class="w-40 -rotate-90" attr:value="60" />
                </div>
                <span class="text-sm text-muted-foreground">"Round"</span>
            </div>

            <div class="flex flex-col gap-3 items-center">
                <div class="flex justify-center items-center h-40">
                    <Slider variant=SliderVariant::Flat class="w-40 -rotate-90" attr:value="30" />
                </div>
                <span class="text-sm text-muted-foreground">"Flat"</span>
            </div>
        </div>
    }
}
```
