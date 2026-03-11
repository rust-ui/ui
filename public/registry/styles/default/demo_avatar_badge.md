---
title: "Demo Avatar Badge"
name: "demo_avatar_badge"
cargo_dependencies: []
registry_dependencies: ["avatar"]
type: "components:demos"
path: "demos/demo_avatar_badge.rs"
---

# Demo Avatar Badge

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_avatar_badge
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::avatar::{Avatar, AvatarBadge, AvatarFallback, AvatarImage};

#[component]
pub fn DemoAvatarBadge() -> impl IntoView {
    view! {
        <div class="flex gap-4 items-center">
            <Avatar>
                <AvatarImage attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=alice" attr:alt="@alice" />
                <AvatarFallback>"AL"</AvatarFallback>
                <AvatarBadge class="bg-green-500 dark:bg-green-700" />
            </Avatar>
            <Avatar>
                <AvatarImage attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=bob" attr:alt="@bob" />
                <AvatarFallback>"BO"</AvatarFallback>
                <AvatarBadge class="bg-red-500 dark:bg-red-700" />
            </Avatar>
            <Avatar>
                <AvatarImage attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=carol" attr:alt="@carol" />
                <AvatarFallback>"CA"</AvatarFallback>
                <AvatarBadge />
            </Avatar>
        </div>
    }
}
```
