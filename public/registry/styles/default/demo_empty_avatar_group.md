---
title: "Demo Empty Avatar Group"
name: "demo_empty_avatar_group"
cargo_dependencies: []
registry_dependencies: ["avatar", "button", "empty"]
type: "components:demos"
path: "demos/demo_empty_avatar_group.rs"
---

# Demo Empty Avatar Group

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_empty_avatar_group
```

## Component Code

```rust
use icons::Plus;
use leptos::prelude::*;

use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarGroup, AvatarImage, AvatarSize};
use crate::components::ui::button::{Button, ButtonSize};
use crate::components::ui::empty::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle};

#[component]
pub fn DemoEmptyAvatarGroup() -> impl IntoView {
    view! {
        <Empty>
            <EmptyHeader>
                <EmptyMedia>
                    <AvatarGroup>
                        <Avatar size=AvatarSize::Lg>
                            <AvatarImage
                                attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=alice"
                                attr:alt="@alice"
                            />
                            <AvatarFallback>"AL"</AvatarFallback>
                        </Avatar>
                        <Avatar size=AvatarSize::Lg>
                            <AvatarImage
                                attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=bob"
                                attr:alt="@bob"
                            />
                            <AvatarFallback>"BO"</AvatarFallback>
                        </Avatar>
                        <Avatar size=AvatarSize::Lg>
                            <AvatarImage
                                attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=carol"
                                attr:alt="@carol"
                            />
                            <AvatarFallback>"CA"</AvatarFallback>
                        </Avatar>
                    </AvatarGroup>
                </EmptyMedia>
                <EmptyTitle>"No Team Members"</EmptyTitle>
                <EmptyDescription>"Invite your team to collaborate on this project."</EmptyDescription>
            </EmptyHeader>
            <EmptyContent>
                <Button size=ButtonSize::Sm>
                    <Plus />
                    "Invite Members"
                </Button>
            </EmptyContent>
        </Empty>
    }
}
```
