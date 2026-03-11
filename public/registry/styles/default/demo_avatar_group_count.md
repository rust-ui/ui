---
title: "Demo Avatar Group Count"
name: "demo_avatar_group_count"
cargo_dependencies: []
registry_dependencies: ["avatar"]
type: "components:demos"
path: "demos/demo_avatar_group_count.rs"
---

# Demo Avatar Group Count

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_avatar_group_count
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarImage};

#[component]
pub fn DemoAvatarGroupCount() -> impl IntoView {
    view! {
        <AvatarGroup>
            <Avatar>
                <AvatarImage attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=alice" attr:alt="@alice" />
                <AvatarFallback>"AL"</AvatarFallback>
            </Avatar>
            <Avatar>
                <AvatarImage attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=bob" attr:alt="@bob" />
                <AvatarFallback>"BO"</AvatarFallback>
            </Avatar>
            <Avatar>
                <AvatarImage attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=carol" attr:alt="@carol" />
                <AvatarFallback>"CA"</AvatarFallback>
            </Avatar>
            <AvatarGroupCount>"+3"</AvatarGroupCount>
        </AvatarGroup>
    }
}
```
