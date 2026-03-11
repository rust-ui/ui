use leptos::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::checkbox::Checkbox;
use crate::ui::field::{
    Field, FieldDescription, FieldGroup, FieldLabel, FieldLegend, FieldSeparator, FieldSet, FieldVariant,
};
use crate::ui::input::Input;

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
