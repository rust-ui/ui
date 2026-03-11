---
title: "Demo Use History"
name: "demo_use_history"
cargo_dependencies: []
registry_dependencies: ["button", "kbd"]
type: "components:demos"
path: "demos/demo_use_history.rs"
---

# Demo Use History

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_use_history
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::hooks::use_history::{UseHistory, use_history};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::kbd::{Kbd, KbdGroup};

const COLORS: &[(&str, &str)] = &[
    ("slate", "bg-slate-500"),
    ("red", "bg-red-500"),
    ("orange", "bg-orange-500"),
    ("green", "bg-green-500"),
    ("blue", "bg-blue-500"),
    ("violet", "bg-violet-500"),
];

#[component]
pub fn DemoUseHistory() -> impl IntoView {
    let _ = UseHistory::init();

    view! { <DemoUseHistoryInner /> }
}

#[component]
fn DemoUseHistoryInner() -> impl IntoView {
    let history = use_history();

    let active = RwSignal::new("slate");

    let can_back = history.can_go_back();
    let can_forward = history.can_go_forward();
    let position = history.position();
    let total = history.total();

    view! {
        <div class="flex flex-col gap-6 items-center">

            // Color swatches
            <div class="flex gap-2 items-center">
                {COLORS
                    .iter()
                    .map(|(name, bg)| {
                        let name = *name;
                        let bg = *bg;
                        view! {
                            <button
                                class=move || {
                                    format!(
                                        "size-8 rounded-full transition-all ring-offset-2 ring-offset-background {} {}",
                                        bg,
                                        if active.get() == name {
                                            "ring-2 ring-foreground scale-110"
                                        } else {
                                            "opacity-60 hover:opacity-100"
                                        },
                                    )
                                }
                                on:click=move |_| {
                                    active.set(name);
                                    history.push(format!("?color={name}"));
                                }
                            />
                        }
                    })
                    .collect_view()}
            </div>

            // Current state
            <p class="text-sm text-muted-foreground">
                "Current: " <span class="font-medium text-foreground">{move || active.get()}</span> "  ·  "
                {move || position.get()} " / " {move || total.get()}
            </p>

            // Undo / Redo buttons
            <div class="flex gap-2 items-center">
                <Button
                    variant=ButtonVariant::Outline
                    size=ButtonSize::Sm
                    attr:disabled=move || !can_back.get()
                    on:click=move |_| history.go_back()
                >
                    "← Undo"
                </Button>
                <Button
                    variant=ButtonVariant::Outline
                    size=ButtonSize::Sm
                    attr:disabled=move || !can_forward.get()
                    on:click=move |_| history.go_forward()
                >
                    "Redo →"
                </Button>
            </div>

            // Keyboard hints
            <div class="flex gap-3 items-center text-xs text-muted-foreground">
                <span class="flex gap-1 items-center">
                    <KbdGroup>
                        <Kbd>"⌘"</Kbd>
                        <Kbd>"Z"</Kbd>
                    </KbdGroup>
                    " undo"
                </span>
                <span class="flex gap-1 items-center">
                    <KbdGroup>
                        <Kbd>"⌘"</Kbd>
                        <Kbd>"⇧"</Kbd>
                        <Kbd>"Z"</Kbd>
                    </KbdGroup>
                    " redo"
                </span>
            </div>

        </div>
    }
}
```
