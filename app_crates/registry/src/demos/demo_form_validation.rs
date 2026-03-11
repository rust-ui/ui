use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::hooks::use_form::use_form;
use crate::ui::form::{Form, FormError, FormField, FormGroup, FormInput, FormLabel, FormProvider, FormSet};

#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate)]
struct FormData {
    #[validate(length(min = 3, message = "Name must be at least 3 characters long."))]
    name: String,
    #[validate(range(min = 18, max = 140, message = "Age must be between 18 and 140."))]
    age: u16,
    #[validate(email(message = "Enter a valid email address."))]
    email: String,
}

#[component]
pub fn DemoFormValidation() -> impl IntoView {
    let form = use_form::<FormData>();

    view! {
        <FormProvider form=form>
            <Form class="max-w-md">
                <FormSet>
                    <FormGroup>
                        <FormField field="name">
                            <FormLabel>Name</FormLabel>
                            <FormInput attr:placeholder="John Doe" />
                            <FormError />
                        </FormField>
                    </FormGroup>
                    <FormGroup>
                        <FormField field="age">
                            <FormLabel>Age</FormLabel>
                            <FormInput attr:r#type="number" attr:placeholder="25" />
                            <FormError />
                        </FormField>
                    </FormGroup>
                    <FormGroup>
                        <FormField field="email">
                            <FormLabel>Email</FormLabel>
                            <FormInput attr:r#type="email" attr:placeholder="example@email.com" />
                            <FormError />
                        </FormField>
                    </FormGroup>
                </FormSet>
            </Form>
        </FormProvider>
    }
}
