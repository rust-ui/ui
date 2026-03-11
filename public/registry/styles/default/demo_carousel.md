---
title: "Demo Carousel"
name: "demo_carousel"
cargo_dependencies: []
registry_dependencies: ["carousel"]
type: "components:demos"
path: "demos/demo_carousel.rs"
---

# Demo Carousel

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_carousel
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::carousel::{Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious};

#[component]
pub fn DemoCarousel() -> impl IntoView {
    view! {
        <div class="px-12 mx-auto w-full max-w-xs">
            <Carousel looping=true>
                <CarouselContent>
                    {(1..=5_u32)
                        .map(|i| {
                            view! {
                                <CarouselItem>
                                    <div class="p-1">
                                        <div class="flex justify-center items-center p-6 rounded-lg border aspect-square bg-card shadow-xs">
                                            <span class="text-4xl font-semibold text-foreground">{i}</span>
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
