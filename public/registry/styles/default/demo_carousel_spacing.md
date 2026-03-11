---
title: "Demo Carousel Spacing"
name: "demo_carousel_spacing"
cargo_dependencies: []
registry_dependencies: ["carousel"]
type: "components:demos"
path: "demos/demo_carousel_spacing.rs"
---

# Demo Carousel Spacing

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_carousel_spacing
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::carousel::{Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious};

#[component]
pub fn DemoCarouselSpacing() -> impl IntoView {
    view! {
        <div class="px-12 mx-auto w-full max-w-sm">
            <Carousel>
                <CarouselContent class="-ml-1">
                    {(1..=5_u32)
                        .map(|i| {
                            view! {
                                <CarouselItem class="pl-1 md:basis-1/2 lg:basis-1/3">
                                    <div class="p-1">
                                        <div class="flex justify-center items-center p-6 rounded-lg border aspect-square bg-card shadow-xs">
                                            <span class="text-2xl font-semibold text-foreground">{i}</span>
                                        </div>
                                    </div>
                                </CarouselItem>
                            }
                        })
                        .collect::<Vec<_>>()}
                </CarouselContent>
                <CarouselPrevious />
                <CarouselNext />
            </Carousel>
        </div>
    }
}
```
