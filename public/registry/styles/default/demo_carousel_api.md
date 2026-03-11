---
title: "Demo Carousel Api"
name: "demo_carousel_api"
cargo_dependencies: []
registry_dependencies: ["carousel"]
type: "components:demos"
path: "demos/demo_carousel_api.rs"
---

# Demo Carousel Api

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_carousel_api
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::carousel::{Carousel, CarouselContent, CarouselIndicator, CarouselItem, CarouselNext, CarouselPrevious};

#[component]
pub fn DemoCarouselApi() -> impl IntoView {
    view! {
        <div class="px-12 mx-auto w-full max-w-xs">
            <Carousel>
                <CarouselContent>
                    {(1..=5_u32)
                        .map(|i| {
                            view! {
                                <CarouselItem>
                                    <div class="flex justify-center items-center p-6 rounded-lg border aspect-square bg-card shadow-xs">
                                        <span class="text-4xl font-semibold text-foreground">{i}</span>
                                    </div>
                                </CarouselItem>
                            }
                        })
                        .collect::<Vec<_>>()}
                </CarouselContent>
                <CarouselPrevious />
                <CarouselNext />
                <CarouselIndicator />
            </Carousel>
        </div>
    }
}
```
