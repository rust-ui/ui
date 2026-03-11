---
title: "Demo Spinner Rtl"
name: "demo_spinner_rtl"
cargo_dependencies: []
registry_dependencies: ["direction_provider", "spinner"]
type: "components:demos"
path: "demos/demo_spinner_rtl.rs"
---

# Demo Spinner Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_spinner_rtl
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::direction_provider::{Direction, DirectionProvider};
use crate::components::ui::spinner::{Spinner, SpinnerCircle};

#[component]
pub fn DemoSpinnerRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <div class="flex gap-4 items-center">
                <Spinner />
                <SpinnerCircle />
                <span class="text-sm text-muted-foreground">"جاري التحميل..."</span>
            </div>
        </DirectionProvider>
    }
}
```
