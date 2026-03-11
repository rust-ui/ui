---
title: "Demo Context Menu Action"
name: "demo_context_menu_action"
cargo_dependencies: []
registry_dependencies: ["context_menu", "separator", "toast_custom"]
type: "components:demos"
path: "demos/demo_context_menu_action.rs"
---

# Demo Context Menu Action

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_context_menu_action
```

## Component Code

```rust
use icons::{Check, Copy, Settings2};
use leptos::prelude::*;

use crate::components::hooks::use_press_hold::use_press_hold;
use crate::components::ui::context_menu::{
    ContextMenu, ContextMenuAction, ContextMenuContent, ContextMenuGroup, ContextMenuItem, ContextMenuLabel,
    ContextMenuTrigger, close_context_menu,
};
use crate::components::ui::separator::Separator;
use crate::components::ui::toast_custom::toast_wrapper::show_toast;

#[component]
pub fn DemoContextMenuAction() -> impl IntoView {
    let on_confirm = Callback::new(move |_| {
        close_context_menu();
        show_toast().success("Action confirmed!");
    });

    view! {
        <ContextMenu>
            <ContextMenuTrigger class="flex justify-center items-center text-sm rounded-md border transition-colors h-[150px] w-[300px] bg-card hover:bg-muted/50">
                "Right click here"
            </ContextMenuTrigger>

            <ContextMenuContent>
                <ContextMenuLabel>"Actions"</ContextMenuLabel>

                <ContextMenuGroup>
                    <ContextMenuItem>
                        <ContextMenuAction>
                            <Copy />
                            "Copy"
                        </ContextMenuAction>
                    </ContextMenuItem>
                    <ContextMenuItem>
                        <ContextMenuAction>
                            <Settings2 />
                            "Settings"
                        </ContextMenuAction>
                    </ContextMenuItem>
                </ContextMenuGroup>

                <Separator class="my-1" />

                <ContextMenuItem class="p-0">
                    <PressHoldAction on_complete=on_confirm />
                </ContextMenuItem>
            </ContextMenuContent>
        </ContextMenu>
    }
}

#[component]
fn PressHoldAction(on_complete: Callback<()>) -> impl IntoView {
    let disabled = Signal::derive(|| false);
    let press_hold = use_press_hold(1500, on_complete, disabled);

    let ph1 = press_hold.clone();
    let ph2 = press_hold.clone();
    let ph3 = press_hold.clone();
    let ph4 = press_hold.clone();

    let progress_style = move || {
        let width_percent = press_hold.progress_signal.get() * 100.0;
        format!(
            "position: absolute; left: 0; top: 0; bottom: 0; width: {width_percent:.1}%; background: rgba(34, 197, 94, 0.3); pointer-events: none; border-radius: inherit;"
        )
    };

    view! {
        <button
            class="flex relative gap-2 items-center py-1.5 px-2 w-full text-sm rounded-sm transition-colors select-none text-primary hover:bg-primary/10"
            on:pointerdown=move |_| ph1.on_pointer_down()
            on:pointerup=move |_| ph2.on_pointer_up()
            on:pointerleave=move |_| ph3.on_pointer_up()
            on:pointercancel=move |_| ph4.on_pointer_up()
        >
            <span style=progress_style></span>
            <span class="flex relative z-10 gap-2 items-center">
                <Check class="size-4" />
                "Hold to Confirm"
            </span>
        </button>
    }
}
```
