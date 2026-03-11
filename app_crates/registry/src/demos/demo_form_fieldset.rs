use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::hooks::use_form::use_form;
use crate::ui::form::{
    Form, FormDescription, FormField, FormGroup, FormInput, FormLabel, FormLegend, FormLegendVariant, FormProvider,
    FormSet,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate)]
struct FormData {
    street: String,
    city: String,
    zip: String,
}

#[component]
pub fn DemoFormFieldset() -> impl IntoView {
    let form = use_form::<FormData>();

    view! {
        <FormProvider form=form>
            <Form class="max-w-md">
                <FormSet>
                    <FormLegend variant=FormLegendVariant::Legend>Address Information</FormLegend>
                    <FormDescription>We need your address to deliver your order.</FormDescription>
                    <FormGroup>
                        <FormField field="street">
                            <FormLabel>Street Address</FormLabel>
                            <FormInput attr:placeholder="123 Main St" />
                        </FormField>
                        <div class="grid grid-cols-2 gap-4">
                            <FormField field="city">
                                <FormLabel>City</FormLabel>
                                <FormInput attr:placeholder="New York" />
                            </FormField>
                            <FormField field="zip">
                                <FormLabel>Postal Code</FormLabel>
                                <FormInput attr:placeholder="90502" />
                            </FormField>
                        </div>
                    </FormGroup>
                </FormSet>
            </Form>
        </FormProvider>
    }
}
