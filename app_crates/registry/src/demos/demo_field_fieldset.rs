use leptos::prelude::*;

use crate::ui::field::{Field, FieldDescription, FieldGroup, FieldLabel, FieldLegend, FieldSet};
use crate::ui::input::Input;

#[component]
pub fn DemoFieldFieldset() -> impl IntoView {
    view! {
        <FieldSet class="w-full max-w-sm">
            <FieldLegend>Address Information</FieldLegend>
            <FieldDescription>We need your address to deliver your order.</FieldDescription>
            <FieldGroup>
                <Field>
                    <FieldLabel html_for="street">Street Address</FieldLabel>
                    <Input attr:id="street" placeholder="123 Main St" />
                </Field>
                <div class="grid grid-cols-2 gap-4">
                    <Field>
                        <FieldLabel html_for="city">City</FieldLabel>
                        <Input attr:id="city" placeholder="New York" />
                    </Field>
                    <Field>
                        <FieldLabel html_for="zip">Postal Code</FieldLabel>
                        <Input attr:id="zip" placeholder="90502" />
                    </Field>
                </div>
            </FieldGroup>
        </FieldSet>
    }
}
