---
title: "Demo Avatar Rtl"
name: "demo_avatar_rtl"
cargo_dependencies: []
registry_dependencies: ["avatar", "direction_provider"]
type: "components:demos"
path: "demos/demo_avatar_rtl.rs"
---

# Demo Avatar Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_avatar_rtl
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoAvatarRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="flex gap-3 items-center max-w-fit">
            <Avatar>
                <AvatarImage attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=rustify" attr:alt="@rustify" />
                <AvatarFallback>"م ع"</AvatarFallback>
            </Avatar>
            <span class="text-sm font-medium">"محمد علي"</span>
        </DirectionProvider>
    }
}
```
