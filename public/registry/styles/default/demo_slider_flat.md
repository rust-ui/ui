---
title: "Demo Slider Flat"
name: "demo_slider_flat"
cargo_dependencies: []
registry_dependencies: ["slider"]
type: "components:demos"
path: "demos/demo_slider_flat.rs"
---

# Demo Slider Flat

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_slider_flat
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::slider::{Slider, SliderVariant};

#[component]
pub fn DemoSliderFlat() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6">
            <Slider variant=SliderVariant::Flat />
            <Slider variant=SliderVariant::Flat attr:min="0" attr:max="100" attr:value="25" attr:step="5" />
            <Slider variant=SliderVariant::Flat attr:disabled="true" attr:value="64" />
        </div>
    }
}
```
