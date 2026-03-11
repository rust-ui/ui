---
title: "Demo Field"
name: "demo_field"
cargo_dependencies: []
registry_dependencies: ["button", "checkbox", "field", "input"]
type: "components:demos"
path: "demos/demo_field.rs"
---

# Demo Field

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_field
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::checkbox::Checkbox;
use crate::components::ui::field::{
    Field, FieldDescription, FieldGroup, FieldLabel, FieldLegend, FieldSeparator, FieldSet, FieldVariant,
};
use crate::components::ui::input::Input;

#[component]
pub fn DemoField() -> impl IntoView {
    view! {
        <div class="w-full max-w-md">
            <FieldGroup>
                <FieldSet>
                    <FieldLegend>Payment Method</FieldLegend>
                    <FieldDescription>All transactions are secure and encrypted.</FieldDescription>
                    <FieldGroup>
                        <Field>
                            <FieldLabel html_for="card-name">Name on Card</FieldLabel>
                            <Input attr:id="card-name" placeholder="Evil Rabbit" />
                        </Field>
                        <Field>
                            <FieldLabel html_for="card-number">Card Number</FieldLabel>
                            <Input attr:id="card-number" placeholder="1234 5678 9012 3456" />
                            <FieldDescription>Enter your 16-digit card number.</FieldDescription>
                        </Field>
                        <div class="grid grid-cols-3 gap-4">
                            <Field>
                                <FieldLabel html_for="exp-month">Month</FieldLabel>
                                <Input attr:id="exp-month" placeholder="MM" />
                            </Field>
                            <Field>
                                <FieldLabel html_for="exp-year">Year</FieldLabel>
                                <Input attr:id="exp-year" placeholder="YYYY" />
                            </Field>
                            <Field>
                                <FieldLabel html_for="cvv">CVV</FieldLabel>
                                <Input attr:id="cvv" placeholder="123" />
                            </Field>
                        </div>
                    </FieldGroup>
                </FieldSet>
                <FieldSeparator />
                <FieldSet>
                    <FieldLegend>Billing Address</FieldLegend>
                    <FieldDescription>The billing address associated with your payment method.</FieldDescription>
                    <FieldGroup>
                        <Field variant=FieldVariant::Horizontal>
                            <Checkbox attr:id="same-as-shipping" checked=true />
                            <FieldLabel html_for="same-as-shipping" class="font-normal">
                                Same as shipping address
                            </FieldLabel>
                        </Field>
                    </FieldGroup>
                </FieldSet>
                <Field variant=FieldVariant::Horizontal>
                    <Button>"Submit"</Button>
                    <Button variant=ButtonVariant::Outline>"Cancel"</Button>
                </Field>
            </FieldGroup>
        </div>
    }
}
```
