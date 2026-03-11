---
title: "Demo Field Rtl"
name: "demo_field_rtl"
cargo_dependencies: []
registry_dependencies: ["button", "checkbox", "direction_provider", "field", "input"]
type: "components:demos"
path: "demos/demo_field_rtl.rs"
---

# Demo Field Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_field_rtl
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::checkbox::Checkbox;
use crate::components::ui::direction_provider::{Direction, DirectionProvider};
use crate::components::ui::field::{
    Field, FieldDescription, FieldGroup, FieldLabel, FieldLegend, FieldSeparator, FieldSet, FieldVariant,
};
use crate::components::ui::input::Input;

#[component]
pub fn DemoFieldRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-md">
            <div class="w-full max-w-md">
                <FieldGroup>
                    <FieldSet>
                        <FieldLegend>"طريقة الدفع"</FieldLegend>
                        <FieldDescription>"جميع المعاملات آمنة ومشفرة."</FieldDescription>
                        <FieldGroup>
                            <Field>
                                <FieldLabel html_for="rtl-card-name">"الاسم على البطاقة"</FieldLabel>
                                <Input attr:id="rtl-card-name" placeholder="محمد علي" />
                            </Field>
                            <Field>
                                <FieldLabel html_for="rtl-card-number">"رقم البطاقة"</FieldLabel>
                                <Input attr:id="rtl-card-number" placeholder="١٢٣٤ ٥٦٧٨ ٩٠١٢ ٣٤٥٦" />
                                <FieldDescription>
                                    "أدخل رقم بطاقتك المكون من ١٦ رقماً."
                                </FieldDescription>
                            </Field>
                            <div class="grid grid-cols-3 gap-4">
                                <Field>
                                    <FieldLabel html_for="rtl-exp-month">"الشهر"</FieldLabel>
                                    <Input attr:id="rtl-exp-month" placeholder="MM" />
                                </Field>
                                <Field>
                                    <FieldLabel html_for="rtl-exp-year">"السنة"</FieldLabel>
                                    <Input attr:id="rtl-exp-year" placeholder="YYYY" />
                                </Field>
                                <Field>
                                    <FieldLabel html_for="rtl-cvv">"CVV"</FieldLabel>
                                    <Input attr:id="rtl-cvv" placeholder="١٢٣" />
                                </Field>
                            </div>
                        </FieldGroup>
                    </FieldSet>
                    <FieldSeparator />
                    <FieldSet>
                        <FieldLegend>"عنوان الفوترة"</FieldLegend>
                        <FieldDescription>
                            "عنوان الفوترة المرتبط بطريقة الدفع الخاصة بك."
                        </FieldDescription>
                        <FieldGroup>
                            <Field variant=FieldVariant::Horizontal>
                                <Checkbox attr:id="rtl-same-as-shipping" checked=true />
                                <FieldLabel html_for="rtl-same-as-shipping" class="font-normal">
                                    "نفس عنوان الشحن"
                                </FieldLabel>
                            </Field>
                        </FieldGroup>
                    </FieldSet>
                    <Field variant=FieldVariant::Horizontal>
                        <Button>"إرسال"</Button>
                        <Button variant=ButtonVariant::Outline>"إلغاء"</Button>
                    </Field>
                </FieldGroup>
            </div>
        </DirectionProvider>
    }
}
```
