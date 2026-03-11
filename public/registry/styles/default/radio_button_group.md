---
title: "Radio Button Group"
name: "radio_button_group"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/radio_button_group.rs"
---

# Radio Button Group

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add radio_button_group
```

## Component Code

```rust
use leptos::prelude::*;
use leptos_ui::{clx, void};

const BASE_BUTTON_GROUP: &str = "flex flex-wrap justify-center mt-2";

mod components {
    use super::*;
    clx! {RadioButtonText, span, "block cursor-pointer bg-transparent text-primary px-3 py-1.5 relative ml-px shadow-[0_0_0_1px_#b5bfd9] tracking-wider text-center transition-colors duration-500"}

    void! {RootInput, input, "radio__button", "focus:outline-0 focus:border-input/60"}
    clx! {RootFieldset, fieldset, BASE_BUTTON_GROUP}
    clx! {RootButtonGroup, div, BASE_BUTTON_GROUP, "[&>label:first-child>span]:rounded-l-md [&>label:last-child>span]:rounded-r-md"}
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn RadioButtonGroup(children: Children) -> impl IntoView {
    view! {
        <RootFieldset>
            <RootButtonGroup attr:role="radio-button-group">{children()}</RootButtonGroup>
        </RootFieldset>
    }
}

#[component]
pub fn RadioButton(children: Children, #[prop(into, optional)] checked: bool) -> impl IntoView {
    view! {
        <label>
            <RootInput attr:r#type="radio" attr:name="radio" attr:checked=checked />
            {children()}
        </label>
    }
}
```
