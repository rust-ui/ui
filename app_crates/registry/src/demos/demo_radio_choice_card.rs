use leptos::prelude::*;

use crate::ui::field::{Field, FieldContent, FieldDescription, FieldLabel, FieldTitle, FieldVariant};
use crate::ui::radio_button::{RadioGroup, RadioGroupItem};

#[component]
pub fn DemoRadioChoiceCard() -> impl IntoView {
    let selected = RwSignal::new("pro".to_string());

    view! {
        <RadioGroup value=selected class="w-full max-w-sm">
            <FieldLabel html_for="plan-plus">
                <Field variant=FieldVariant::Horizontal>
                    <FieldContent>
                        <FieldTitle>"Plus"</FieldTitle>
                        <FieldDescription>"For individuals and small teams."</FieldDescription>
                    </FieldContent>
                    <RadioGroupItem value="plus" id="plan-plus" />
                </Field>
            </FieldLabel>
            <FieldLabel html_for="plan-pro">
                <Field variant=FieldVariant::Horizontal>
                    <FieldContent>
                        <FieldTitle>"Pro"</FieldTitle>
                        <FieldDescription>"For growing businesses."</FieldDescription>
                    </FieldContent>
                    <RadioGroupItem value="pro" id="plan-pro" />
                </Field>
            </FieldLabel>
            <FieldLabel html_for="plan-enterprise">
                <Field variant=FieldVariant::Horizontal>
                    <FieldContent>
                        <FieldTitle>"Enterprise"</FieldTitle>
                        <FieldDescription>"For large teams and enterprises."</FieldDescription>
                    </FieldContent>
                    <RadioGroupItem value="enterprise" id="plan-enterprise" />
                </Field>
            </FieldLabel>
        </RadioGroup>
    }
}
