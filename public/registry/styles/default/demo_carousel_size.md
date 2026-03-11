---
title: "Demo Carousel Size"
name: "demo_carousel_size"
cargo_dependencies: []
registry_dependencies: ["carousel"]
type: "components:demos"
path: "demos/demo_carousel_size.rs"
---

# Demo Carousel Size

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_carousel_size
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::carousel::{Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious};

#[component]
pub fn DemoCarouselSize() -> impl IntoView {
    view! {
        <div class="px-12 mx-auto w-full max-w-sm">
            <Carousel>
                <CarouselContent>
                    {(1..=5_u32)
                        .map(|i| {
                            view! {
                                <CarouselItem class="md:basis-1/2 lg:basis-1/3">
                                    <div class="p-1">
                                        <div class="flex justify-center items-center p-6 rounded-lg border aspect-square bg-card shadow-xs">
                                            <span class="text-3xl font-semibold text-foreground">{i}</span>
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
