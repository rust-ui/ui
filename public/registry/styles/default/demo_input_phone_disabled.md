---
title: "Demo Input Phone Disabled"
name: "demo_input_phone_disabled"
cargo_dependencies: []
registry_dependencies: ["input_phone"]
type: "components:demos"
path: "demos/demo_input_phone_disabled.rs"
---

# Demo Input Phone Disabled

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_input_phone_disabled
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::input_phone::InputPhone;
use crate::utils::country::Country;
use crate::utils::phone_number::PhoneNumber;

#[component]
pub fn DemoInputPhoneDisabled() -> impl IntoView {
    let phone_signal = RwSignal::new(PhoneNumber::new("0612345678", 10));
    let country_signal = RwSignal::new(Country::France);

    view! {
        <div class="flex flex-col gap-4 w-full max-w-sm">
            <div class="space-y-2">
                <label class="text-sm font-medium">"Disabled"</label>
                <InputPhone value_signal=phone_signal country_signal=country_signal disabled=true />
            </div>
        </div>
    }
}
```
