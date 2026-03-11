---
title: "Demo Range Slider"
name: "demo_range_slider"
cargo_dependencies: []
registry_dependencies: []
type: "components:demos"
path: "demos/demo_range_slider.rs"
---

# Demo Range Slider

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_range_slider
```

## Component Code

```rust
use leptos::prelude::*;

#[component]
pub fn DemoRangeSlider() -> impl IntoView {
    view! {
        <div class="p-6 w-full max-w-md bg-white rounded-lg shadow-lg">
            <h2 class="mb-4 text-2xl font-bold">Range Slider</h2>
            <div class="mb-4">
                <label for="price-range" class="block mb-2 font-bold text-gray-700">
                    Price Range
                </label>
                <input
                    type="range"
                    id="price-range"
                    class="w-full accent-indigo-600"
                    min="0"
                    max="1000"
                    value="500"
                    oninput="updatePrice(this.value)"
                />
            </div>
            <div class="flex justify-between text-gray-500">
                <span id="minPrice">$0</span>
                <span id="maxPrice">$1000</span>
            </div>
        </div>

        <script src="/components/range_slider.js" />
    }
}
```
