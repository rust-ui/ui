---
title: "Demo Slider Rtl"
name: "demo_slider_rtl"
cargo_dependencies: []
registry_dependencies: ["direction_provider", "slider"]
type: "components:demos"
path: "demos/demo_slider_rtl.rs"
---

# Demo Slider Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_slider_rtl
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::direction_provider::{Direction, DirectionProvider};
use crate::components::ui::slider::Slider;

#[component]
pub fn DemoSliderRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-sm">
            <div class="flex flex-col gap-6 w-full max-w-sm">
                <Slider />
                <Slider attr:min="0" attr:max="100" attr:value="40" attr:step="5" />
                <Slider attr:disabled="true" attr:value="80" />
            </div>
        </DirectionProvider>
    }
}
```
