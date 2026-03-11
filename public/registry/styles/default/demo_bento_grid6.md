---
title: "Demo Bento Grid6"
name: "demo_bento_grid6"
cargo_dependencies: []
registry_dependencies: ["bento_grid"]
type: "components:demos"
path: "demos/demo_bento_grid6.rs"
---

# Demo Bento Grid6

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_bento_grid6
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::bento_grid::{BentoCell, BentoGrid6, BentoRow};

#[component]
pub fn DemoBentoGrid6() -> impl IntoView {
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
            <BentoGrid6>
                <BentoRow class="">
                    <BentoCell>1</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-span-2 md:row-span-2 md:h-full">
                    <BentoCell>2</BentoCell>
                </BentoRow>
                <BentoRow class="">
                    <BentoCell>3</BentoCell>
                </BentoRow>
                <BentoRow class="">
                    <BentoCell>4</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-start-4">
                    <BentoCell>5</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-span-4">
                    <BentoCell>6</BentoCell>
                </BentoRow>
            </BentoGrid6>
        </div>
    }
}

#[component]
pub fn Variant2() -> impl IntoView {
    view! {
        <div>

            <h4 class="text-xl font-bold text-pretty">Variant 2</h4>
            <BentoGrid6>
                <div class="row-span-2 p-1 h-full rounded-lg">
                    <BentoCell>1</BentoCell>
                </div>
                <BentoRow class="">
                    <BentoCell>2</BentoCell>
                </BentoRow>
                <div class="row-span-2 p-1 h-full rounded-lg">
                    <BentoCell>3</BentoCell>
                </div>
                <BentoRow class="">
                    <BentoCell>4</BentoCell>
                </BentoRow>
                <BentoRow class="">
                    <BentoCell>5</BentoCell>
                </BentoRow>
                <BentoRow class="">
                    <BentoCell>6</BentoCell>
                </BentoRow>
            </BentoGrid6>

        </div>
    }
}

#[component]
pub fn Variant3() -> impl IntoView {
    view! {
        <div>

            <h4 class="text-xl font-bold text-pretty">Variant 3</h4>
            <BentoGrid6>
                <BentoRow class="">
                    <BentoCell>1</BentoCell>
                </BentoRow>
                <BentoRow class="">
                    <BentoCell>2</BentoCell>
                </BentoRow>
                <div class="col-span-2 p-1 h-32 rounded-lg">
                    <BentoCell>3</BentoCell>
                </div>
                <div class="col-span-2 p-1 h-32 rounded-lg">
                    <BentoCell>4</BentoCell>
                </div>
                <BentoRow class="">
                    <BentoCell>5</BentoCell>
                </BentoRow>
                <BentoRow class="">
                    <BentoCell>6</BentoCell>
                </BentoRow>
            </BentoGrid6>

        </div>
    }
}

#[component]
pub fn Variant4() -> impl IntoView {
    view! {
        <div>
            <h4 class="text-xl font-bold text-pretty">Variant 4</h4>
            <div class="grid gap-2 sm:grid-cols-2 md:grid-cols-6">
                <BentoRow class="md:col-span-3">
                    <BentoCell>1</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-span-3">
                    <BentoCell>2</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-span-4">
                    <BentoCell>3</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-span-2">
                    <BentoCell>4</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-span-2">
                    <BentoCell>5</BentoCell>
                </BentoRow>
                <BentoRow class="md:col-span-4">
                    <BentoCell>6</BentoCell>
                </BentoRow>
            </div>
        </div>
    }
}
```
