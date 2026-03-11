---
title: "Radio Button"
name: "radio_button"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/radio_button.rs"
---

# Radio Button

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add radio_button
```

## Component Code

```rust
use leptos::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn RadioGroup(#[prop(into, optional)] class: String, value: RwSignal<String>, children: Children) -> impl IntoView {
    provide_context(value);

    let class = tw_merge!("flex flex-col gap-3", class);

    view! {
        <div data-name="RadioGroup" class=class role="radiogroup">
            {children()}
        </div>
    }
}

#[component]
pub fn RadioGroupItem(
    #[prop(into, optional)] class: String,
    #[prop(into)] value: String,
    #[prop(into, optional)] id: String,
    #[prop(into, optional)] disabled: Signal<bool>,
) -> impl IntoView {
    let selected = expect_context::<RwSignal<String>>();
    let value_for_check = value.clone();
    let value_for_click = value;

    let is_checked = Memo::new(move |_| selected.get() == value_for_check);

    let radio_class = tw_merge!(
        "aspect-square size-4 shrink-0 rounded-full border border-input shadow-xs transition-colors",
        "focus:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2",
        "disabled:cursor-not-allowed disabled:opacity-50",
        "data-[state=checked]:border-primary",
        class
    );

    view! {
        <button
            data-name="RadioGroupItem"
            type="button"
            role="radio"
            id=id
            class=radio_class
            aria-checked=move || is_checked.get().to_string()
            data-state=move || if is_checked.get() { "checked" } else { "unchecked" }
            disabled=move || disabled.get()
            on:click=move |_| {
                if !disabled.get() {
                    selected.set(value_for_click.clone());
                }
            }
        >
            <span class="flex justify-center items-center">
                {move || is_checked.get().then(|| view! { <span class="rounded-full size-2.5 bg-primary"></span> })}
            </span>
        </button>
    }
}
```
