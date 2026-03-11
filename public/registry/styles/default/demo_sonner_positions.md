---
title: "Demo Sonner Positions"
name: "demo_sonner_positions"
cargo_dependencies: []
registry_dependencies: ["sonner"]
type: "components:demos"
path: "demos/demo_sonner_positions.rs"
---

# Demo Sonner Positions

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_sonner_positions
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::sonner::{SonnerPosition, SonnerToaster, SonnerTrigger};

#[component]
pub fn DemoSonnerPositions() -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-2 justify-center">
            <SonnerTrigger
                title="Top Left"
                description="Toast positioned at top-left"
                position="TopLeft"
                class="bg-secondary text-secondary-foreground hover:bg-secondary/80"
            >
                "Top Left"
            </SonnerTrigger>
            <SonnerTrigger
                title="Top Center"
                description="Toast positioned at top-center"
                position="TopCenter"
                class="bg-secondary text-secondary-foreground hover:bg-secondary/80"
            >
                "Top Center"
            </SonnerTrigger>
            <SonnerTrigger
                title="Top Right"
                description="Toast positioned at top-right"
                position="TopRight"
                class="bg-secondary text-secondary-foreground hover:bg-secondary/80"
            >
                "Top Right"
            </SonnerTrigger>
            <SonnerTrigger
                title="Bottom Left"
                description="Toast positioned at bottom-left"
                position="BottomLeft"
                class="bg-secondary text-secondary-foreground hover:bg-secondary/80"
            >
                "Bottom Left"
            </SonnerTrigger>
            <SonnerTrigger
                title="Bottom Center"
                description="Toast positioned at bottom-center"
                position="BottomCenter"
                class="bg-secondary text-secondary-foreground hover:bg-secondary/80"
            >
                "Bottom Center"
            </SonnerTrigger>
            <SonnerTrigger
                title="Bottom Right"
                description="Toast positioned at bottom-right (default)"
                position="BottomRight"
            >
                "Bottom Right (Default)"
            </SonnerTrigger>
        </div>

        // Toasters at each position
        <SonnerToaster position=SonnerPosition::TopLeft />
        <SonnerToaster position=SonnerPosition::TopCenter />
        <SonnerToaster position=SonnerPosition::TopRight />
        <SonnerToaster position=SonnerPosition::BottomLeft />
        <SonnerToaster position=SonnerPosition::BottomCenter />
    }
}
```
