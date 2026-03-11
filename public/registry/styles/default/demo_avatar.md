---
title: "Demo Avatar"
name: "demo_avatar"
cargo_dependencies: []
registry_dependencies: ["avatar"]
type: "components:demos"
path: "demos/demo_avatar.rs"
---

# Demo Avatar

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_avatar
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};

#[component]
pub fn DemoAvatar() -> impl IntoView {
    view! {
        <Avatar>
            <AvatarImage attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=rustify" attr:alt="@rustify" />
            <AvatarFallback>"RS"</AvatarFallback>
        </Avatar>
    }
}
```
