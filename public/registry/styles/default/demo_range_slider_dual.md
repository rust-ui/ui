---
title: "Demo Range Slider Dual"
name: "demo_range_slider_dual"
cargo_dependencies: []
registry_dependencies: []
type: "components:demos"
path: "demos/demo_range_slider_dual.rs"
---

# Demo Range Slider Dual

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_range_slider_dual
```

## Component Code

```rust
use leptos::prelude::*;

#[component]
pub fn DemoRangeSliderDual() -> impl IntoView {
    view! {
        <link rel="stylesheet" href="/components/range_slider_dual.css" />
        <script src="/components/range_slider_dual.js" />

        <div class="flex justify-center items-center w-full bg-gray-100 h-[300px]">
            <div class="p-6 bg-white rounded-lg shadow-lg w-[400px]">
                <h2 class="text-lg font-bold">PRICE RANGE</h2>

                <div class="relative mt-4 slider-container">

                    <input type="range" id="minRange" min="0" max="400" value="0" />
                    <input type="range" id="maxRange" min="0" max="400" value="400" />

                    <div class="relative w-full h-2 bg-gray-200 rounded-md">
                        <div
                            id="rangeTrack"
                            class="absolute h-2 bg-gradient-to-r from-blue-900 to-blue-400 rounded-md"
                        ></div>
                    </div>
                </div>

                <div class="flex justify-between mt-3 text-gray-600">
                    <span>Min Price: $<span id="minValue">0</span></span>
                    <span>Max Price: $<span id="maxValue">400</span></span>
                </div>
            </div>

        </div>
    }
}
```
