---
title: "Checkbox"
name: "checkbox"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/checkbox.rs"
description: "Rust/UI component that displays a control that allows the user to toggle between checked and not checked."
tags: ["utils"]
---

# Checkbox

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add checkbox
```

## Component Code

```rust
use icons::Check;
use leptos::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Checkbox(
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] checked: Signal<bool>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] on_checked_change: Option<Callback<bool>>,
    #[prop(into, optional, default = "Checkbox".to_string())] aria_label: String,
) -> impl IntoView {
    let checked_state = move || if checked.get() { "checked" } else { "unchecked" };

    let checkbox_class = tw_merge!(
        "peer border-input dark:bg-input/30 data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground dark:data-[state=checked]:bg-primary data-[state=checked]:border-primary focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive size-4 shrink-0 rounded-[4px] border shadow-xs transition-shadow outline-none focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50",
        class
    );

    view! {
        <button
            data-name="Checkbox"
            class=checkbox_class
            data-state=checked_state
            type="button"
            role="checkbox"
            aria-checked=move || checked.get().to_string()
            aria-label=aria_label
            disabled=move || disabled.get()
            on:click=move |_| {
                if !disabled.get() && let Some(callback) = on_checked_change {
                    callback.run(!checked.get());
                }
            }
        >
            <span data-name="CheckboxIndicator" class="flex justify-center items-center text-current transition-none">
                {move || { checked.get().then(|| view! { <Check class="size-3.5".to_string() /> }) }}
            </span>
        </button>
    }
}
```
