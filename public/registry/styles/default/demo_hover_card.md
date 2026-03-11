---
title: "Demo Hover Card"
name: "demo_hover_card"
cargo_dependencies: []
registry_dependencies: ["avatar", "button", "hover_card"]
type: "components:demos"
path: "demos/demo_hover_card.rs"
---

# Demo Hover Card

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_hover_card
```

## Component Code

```rust
use icons::Calendar;
use leptos::prelude::*;

use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::hover_card::{HoverCard, HoverCardContent, HoverCardTrigger};

#[component]
pub fn DemoHoverCard() -> impl IntoView {
    view! {
        <HoverCard>
            <HoverCardTrigger>
                <Button variant=ButtonVariant::Link>"@rust-lang"</Button>
            </HoverCardTrigger>
            <HoverCardContent class="w-80">
                <div class="flex gap-4">
                    <Avatar>
                        <AvatarImage
                            attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=rust-lang"
                            attr:alt="@rust-lang"
                        />
                        <AvatarFallback>"RL"</AvatarFallback>
                    </Avatar>
                    <div class="flex flex-col gap-1">
                        <h4 class="text-sm font-semibold">"@rust-lang"</h4>
                        <p class="text-sm text-muted-foreground">
                            "Empowering everyone to build reliable and efficient software."
                        </p>
                        <div class="flex gap-2 items-center mt-2">
                            <Calendar class="size-3.5 text-muted-foreground" />
                            <span class="text-xs text-muted-foreground">"Active since 2010"</span>
                        </div>
                    </div>
                </div>
            </HoverCardContent>
        </HoverCard>
    }
}
```
