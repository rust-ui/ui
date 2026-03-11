---
title: "Toaster"
name: "toaster"
cargo_dependencies: []
registry_dependencies: ["toast_custom"]
type: "components:ui/toast_custom"
path: "ui/toaster.rs"
---

# Toaster

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add toaster
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::toast_custom::_context::ToasterContext;
use crate::components::ui::toast_custom::_data::{ToastData, ToastPosition};
use crate::components::ui::toast_custom::_template_styles::TEMPLATE_STYLES;
use crate::components::ui::toast_custom::toast::Toast;

const CONTAINER_POSITIONS: &[ToastPosition] =
    &[ToastPosition::TopLeft, ToastPosition::TopRight, ToastPosition::BottomRight, ToastPosition::BottomLeft];

#[component]
pub fn Toaster(#[prop(optional, into)] stacked: Signal<bool>) -> impl IntoView {
    let toaster = expect_toaster();

    view! {
        <style>{TEMPLATE_STYLES}</style>

        <For each=move || CONTAINER_POSITIONS key=|position| get_container_id(position) let:position>
            <Show when=move || !is_container_empty(position)>
                <div
                    class=get_container_class(stacked.get(), position)
                    style:width="var(--leptoaster-width)"
                    style:max-width="var(--leptoaster-max-width)"
                    style:margin=get_container_margin(position)
                    style:position="fixed"
                    style:inset=get_container_inset(position)
                    style:z-index="var(--leptoaster-z-index)"
                >
                    <For
                        each=move || {
                            let toasts = toaster.queue_signal.get();
                            match position {
                                ToastPosition::BottomLeft | ToastPosition::BottomRight => {
                                    toasts
                                        .iter()
                                        .filter(|toast| toast.position.eq(position))
                                        .cloned()
                                        .collect::<Vec<ToastData>>()
                                }
                                ToastPosition::TopLeft | ToastPosition::TopRight => {
                                    toasts
                                        .iter()
                                        .filter(|toast| toast.position.eq(position))
                                        .cloned()
                                        .rev()
                                        .collect::<Vec<ToastData>>()
                                }
                            }
                        }
                        key=|toast| toast.id
                        let:toast
                    >
                        <Toast toast=toast />
                    </For>
                </div>
            </Show>
        </For>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

pub fn provide_toaster() {
    if use_context::<ToasterContext>().is_none() {
        provide_context(ToasterContext::default());
    }
}

#[must_use]
pub fn expect_toaster() -> ToasterContext {
    expect_context::<ToasterContext>()
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

fn is_container_empty(position: &ToastPosition) -> bool {
    !expect_toaster().queue_signal.get().iter().any(|toast| toast.position.eq(position))
}

fn get_container_id(position: &ToastPosition) -> &'static str {
    match position {
        ToastPosition::TopLeft => "top_left",
        ToastPosition::TopRight => "top_right",
        ToastPosition::BottomRight => "bottom_right",
        ToastPosition::BottomLeft => "bottom_left",
    }
}

fn get_container_inset(position: &ToastPosition) -> &'static str {
    match position {
        ToastPosition::TopLeft => "0 auto auto 0",
        ToastPosition::TopRight => "0 0 auto auto",
        ToastPosition::BottomRight => "auto 0 0 auto",
        ToastPosition::BottomLeft => "auto 0 0 0",
    }
}

fn get_container_margin(position: &ToastPosition) -> &'static str {
    match position {
        ToastPosition::TopLeft | ToastPosition::BottomLeft => "0 0 0 12px",
        ToastPosition::TopRight | ToastPosition::BottomRight => "0 12px 0 0",
    }
}

fn get_container_class(stacked: bool, position: &ToastPosition) -> Option<&'static str> {
    if !stacked {
        return None;
    }

    match position {
        ToastPosition::BottomLeft | ToastPosition::BottomRight => Some("leptoaster-stack-container-bottom"),

        ToastPosition::TopLeft | ToastPosition::TopRight => Some("leptoaster-stack-container-top"),
    }
}
```
