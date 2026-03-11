---
title: "Demo Avatar Badge Icon"
name: "demo_avatar_badge_icon"
cargo_dependencies: []
registry_dependencies: ["avatar"]
type: "components:demos"
path: "demos/demo_avatar_badge_icon.rs"
---

# Demo Avatar Badge Icon

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_avatar_badge_icon
```

## Component Code

```rust
use icons::Plus;
use leptos::prelude::*;

use crate::components::ui::avatar::{Avatar, AvatarBadge, AvatarFallback, AvatarImage};

#[component]
pub fn DemoAvatarBadgeIcon() -> impl IntoView {
    view! {
        <Avatar>
            <AvatarImage attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=rustify" attr:alt="@rustify" />
            <AvatarFallback>"RS"</AvatarFallback>
            <AvatarBadge>
                <Plus />
            </AvatarBadge>
        </Avatar>
    }
}
```
