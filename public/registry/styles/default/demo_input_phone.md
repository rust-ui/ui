---
title: "Demo Input Phone"
name: "demo_input_phone"
cargo_dependencies: []
registry_dependencies: ["input_phone"]
type: "components:demos"
path: "demos/demo_input_phone.rs"
---

# Demo Input Phone

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_input_phone
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::input_phone::InputPhone;
use crate::utils::country::Country;
use crate::utils::phone_number::PhoneNumber;

#[component]
pub fn DemoInputPhone() -> impl IntoView {
    let phone_signal = RwSignal::new(PhoneNumber::default());
    let country_signal = RwSignal::new(Country::UnitedStatesOfAmerica);

    view! {
        <div class="flex flex-col gap-2 w-full max-w-sm">
            <InputPhone value_signal=phone_signal country_signal=country_signal />
            <p class="text-sm text-muted-foreground">
                {move || {
                    let phone = phone_signal.get();
                    let country = country_signal.get();
                    if phone.is_empty() {
                        "Enter a phone number".to_string()
                    } else {
                        phone.format_international(country)
                    }
                }}
            </p>
        </div>
    }
}
```
