---
title: "Demo Bento Grid4"
name: "demo_bento_grid4"
cargo_dependencies: []
registry_dependencies: ["bento_grid"]
type: "components:demos"
path: "demos/demo_bento_grid4.rs"
---

# Demo Bento Grid4

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_bento_grid4
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::bento_grid::{BentoCell, BentoGrid, BentoRow};

#[component]
pub fn DemoBentoGrid4() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6 py-4 w-full max-w-[800px]">
            <Variant1 />
            <Variant2 />
            <Variant3 />
            <Variant4 />
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn Variant1() -> impl IntoView {
    view! {
        <div>
            <h4 class="text-xl font-bold text-pretty">Variant 1</h4>

            <BentoGrid>
                <BentoRow class="md:col-span-3">
                    <BentoCell>1</BentoCell>
                </BentoRow>
                <BentoRow class="">
                    <BentoCell>2</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-start-1">
                    <BentoCell>3</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-span-4 md:col-start-2">
                    <BentoCell>4</BentoCell>
                </BentoRow>
            </BentoGrid>
        </div>
    }
}

#[component]
pub fn Variant2() -> impl IntoView {
    view! {
        <div>
            <h4 class="text-xl font-bold text-pretty">Variant 2</h4>

            <BentoGrid>
                <BentoRow class="md:col-span-3">
                    <BentoCell>1</BentoCell>
                </BentoRow>
                <BentoRow class="">
                    <BentoCell>2</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-span-2">
                    <BentoCell>3</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-span-2">
                    <BentoCell>4</BentoCell>
                </BentoRow>
            </BentoGrid>

        </div>
    }
}

#[component]
pub fn Variant3() -> impl IntoView {
    view! {
        <div>
            <h4 class="text-xl font-bold text-pretty">Variant 3</h4>

            <BentoGrid>
                <BentoRow class="md:col-start-1">
                    <BentoCell>1</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-span-2">
                    <BentoCell>2</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-start-4">
                    <BentoCell>3</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-span-4">
                    <BentoCell>4</BentoCell>
                </BentoRow>
            </BentoGrid>

        </div>
    }
}

#[component]
pub fn Variant4() -> impl IntoView {
    view! {
        <div>
            <h4 class="text-xl font-bold text-pretty">Variant 4</h4>

            <BentoGrid>
                <BentoRow class="md:col-span-3 md:row-span-4 md:h-full">
                    <BentoCell>1</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-start-4">
                    <BentoCell>2</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-start-4">
                    <BentoCell>3</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-start-4">
                    <BentoCell>4</BentoCell>
                </BentoRow>
            </BentoGrid>

        </div>
    }
}
```
