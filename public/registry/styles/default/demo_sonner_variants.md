---
title: "Demo Sonner Variants"
name: "demo_sonner_variants"
cargo_dependencies: []
registry_dependencies: ["sonner"]
type: "components:demos"
path: "demos/demo_sonner_variants.rs"
---

# Demo Sonner Variants

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_sonner_variants
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::sonner::{SonnerTrigger, ToastType};

#[component]
pub fn DemoSonnerVariants() -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-2 justify-center">
            <SonnerTrigger title="You got a message" description="You toasted me!">
                "Default"
            </SonnerTrigger>

            <SonnerTrigger title="You got a message" description="You toasted me!" variant=ToastType::Success>
                "Success"
            </SonnerTrigger>
            <SonnerTrigger title="You got a message" description="You toasted me!" variant=ToastType::Error>
                "Error"
            </SonnerTrigger>
            <SonnerTrigger title="You got a message" description="You toasted me!" variant=ToastType::Warning>
                "Warning"
            </SonnerTrigger>
            <SonnerTrigger title="You got a message" description="You toasted me!" variant=ToastType::Info>
                "Info"
            </SonnerTrigger>
            <SonnerTrigger title="You got a message" description="You toasted me!" variant=ToastType::Loading>
                "Loading"
            </SonnerTrigger>
        </div>
    }
}
```
