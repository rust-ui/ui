---
title: "Demo Checkbox Rtl"
name: "demo_checkbox_rtl"
cargo_dependencies: []
registry_dependencies: ["checkbox", "direction_provider", "label"]
type: "components:demos"
path: "demos/demo_checkbox_rtl.rs"
---

# Demo Checkbox Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_checkbox_rtl
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::checkbox::Checkbox;
use crate::components::ui::direction_provider::{Direction, DirectionProvider};
use crate::components::ui::label::Label;

#[component]
pub fn DemoCheckboxRtl() -> impl IntoView {
    let item1_signal = RwSignal::new(true);
    let item2_signal = RwSignal::new(false);

    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-sm">
            <div class="flex flex-col gap-4">
                <div class="flex gap-2 items-center">
                    <Checkbox
                        attr:id="rtl-item1"
                        checked=item1_signal
                        on_checked_change=Callback::new(move |checked| {
                            item1_signal.set(checked);
                        })
                    />
                    <Label html_for="rtl-item1">"قبول الشروط والأحكام"</Label>
                </div>
                <div class="flex gap-2 items-center">
                    <Checkbox
                        attr:id="rtl-item2"
                        checked=item2_signal
                        on_checked_change=Callback::new(move |checked| {
                            item2_signal.set(checked);
                        })
                    />
                    <Label html_for="rtl-item2">"الاشتراك في النشرة البريدية"</Label>
                </div>
                <div class="flex gap-2 items-center">
                    <Checkbox attr:id="rtl-disabled1" checked=true disabled=true />
                    <Label html_for="rtl-disabled1">"معطّل (محدد)"</Label>
                </div>
                <div class="flex gap-2 items-center">
                    <Checkbox attr:id="rtl-disabled2" disabled=true />
                    <Label html_for="rtl-disabled2">"معطّل"</Label>
                </div>
            </div>
        </DirectionProvider>
    }
}
```
