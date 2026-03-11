---
title: "Demo Use Locks"
name: "demo_use_locks"
cargo_dependencies: []
registry_dependencies: ["button"]
type: "components:demos"
path: "demos/demo_use_locks.rs"
---

# Demo Use Locks

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_use_locks
```

## Component Code

```rust
use icons::{Lock, LockOpen};
use leptos::prelude::*;

use crate::components::hooks::use_locks::{LockableParam, UseLocks, use_locks};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn DemoUseLocks() -> impl IntoView {
    let _ = UseLocks::init();

    view! { <DemoUseLocksInner /> }
}

#[component]
fn DemoUseLocksInner() -> impl IntoView {
    let locks = use_locks();

    view! {
        <div class="flex flex-col gap-4 my-4 w-full max-w-sm">

            // Param rows
            {LockableParam::ALL
                .iter()
                .map(|&param| {
                    let is_locked = locks.is_locked(param);
                    let variant = Signal::derive(move || {
                        if is_locked.get() { ButtonVariant::Default } else { ButtonVariant::Ghost }
                    });
                    view! {
                        <div class="flex gap-3 justify-between items-center py-2 px-3 rounded-md border">
                            <span class=move || {
                                format!(
                                    "text-sm {}",
                                    if is_locked.get() {
                                        "text-muted-foreground line-through"
                                    } else {
                                        "text-foreground"
                                    },
                                )
                            }>{param.label()}</span>
                            <Button variant=variant size=ButtonSize::Icon on:click=move |_| locks.toggle_lock(param)>
                                {move || {
                                    if is_locked.get() {
                                        view! { <Lock class="size-3.5" /> }.into_any()
                                    } else {
                                        view! { <LockOpen class="size-3.5" /> }.into_any()
                                    }
                                }}
                            </Button>
                        </div>
                    }
                })
                .collect_view()} // Summary
            <p class="pt-1 text-xs text-center text-muted-foreground">
                {move || {
                    let count = locks.locked_params().len();
                    if count == 0 {
                        "No params locked — all will be randomized".to_string()
                    } else {
                        format!("{count} param{} locked", if count == 1 { "" } else { "s" })
                    }
                }}
            </p>

        </div>
    }
}
```
