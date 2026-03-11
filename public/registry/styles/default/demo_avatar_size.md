---
title: "Demo Avatar Size"
name: "demo_avatar_size"
cargo_dependencies: []
registry_dependencies: ["avatar"]
type: "components:demos"
path: "demos/demo_avatar_size.rs"
---

# Demo Avatar Size

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_avatar_size
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage, AvatarSize};

#[component]
pub fn DemoAvatarSize() -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-4 items-center">
            <Avatar size=AvatarSize::Sm>
                <AvatarImage attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=rustify" attr:alt="@rustify" />
                <AvatarFallback>"RS"</AvatarFallback>
            </Avatar>
            <Avatar>
                <AvatarImage attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=rustify" attr:alt="@rustify" />
                <AvatarFallback>"RS"</AvatarFallback>
            </Avatar>
            <Avatar size=AvatarSize::Lg>
                <AvatarImage attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=rustify" attr:alt="@rustify" />
                <AvatarFallback>"RS"</AvatarFallback>
            </Avatar>
        </div>
    }
}
```
