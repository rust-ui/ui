---
title: "Demo Slider"
name: "demo_slider"
cargo_dependencies: []
registry_dependencies: ["slider"]
type: "components:demos"
path: "demos/demo_slider.rs"
---

# Demo Slider

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_slider
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::slider::Slider;

#[component]
pub fn DemoSlider() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6">
            <Slider />
            <Slider attr:min="0" attr:max="100" attr:value="40" attr:step="5" />
            <Slider attr:disabled="true" attr:value="80" />
        </div>
    }
}
```
