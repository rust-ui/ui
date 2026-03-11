---
title: "Demo Card Reverse"
name: "demo_card_reverse"
cargo_dependencies: []
registry_dependencies: ["card"]
type: "components:demos"
path: "demos/demo_card_reverse.rs"
---

# Demo Card Reverse

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_card_reverse
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::card::{Card, CardDescription, CardTitle};

#[component]
pub fn DemoCardReverse() -> impl IntoView {
    view! {
        <div class="space-y-6">
            // Card 1 - Normal layout
            <Card class="flex flex-col gap-6 px-6 md:flex-row">
                <div class="w-full h-48 rounded-lg md:w-1/3 md:h-32 bg-muted" />

                <div class="flex-1 pt-0">
                    <CardTitle class="mb-3">"Nature's Beauty"</CardTitle>
                    <CardDescription>
                        "Nature's beauty encompasses a vast array of colors, sounds, and textures that evoke a sense of wonder. Its rhythms and patterns create a calming atmosphere that can rejuvenate the spirit."
                    </CardDescription>
                </div>
            </Card>

            // Card 2 - Reverse layout
            <Card class="flex flex-col gap-6 px-6 md:flex-row-reverse">
                <div class="w-full h-48 rounded-lg md:w-1/3 md:h-32 bg-accent" />

                <div class="flex-1 pt-0">
                    <CardTitle class="mb-3">"Ecosystem Balance"</CardTitle>
                    <CardDescription>
                        "The intricate balance of ecosystems showcases the interdependence of all living beings. Each element, from the smallest insect to the largest tree, plays a vital role in sustaining life."
                    </CardDescription>
                </div>
            </Card>

            // Card 3 - Normal layout
            <Card class="flex flex-col gap-6 px-6 md:flex-row">
                <div class="w-full h-48 rounded-lg md:w-1/3 md:h-32 bg-muted" />

                <div class="flex-1 pt-0">
                    <CardTitle class="mb-3">"Changing Landscapes"</CardTitle>
                    <CardDescription>
                        "The ever-changing landscapes of nature remind us of the passage of time. Seasons bring transformations that create a dynamic environment filled with diverse flora and fauna."
                    </CardDescription>
                </div>
            </Card>
        </div>
    }
}
```
