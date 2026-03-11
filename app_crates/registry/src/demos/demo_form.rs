use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::hooks::use_form::use_form;
use crate::ui::form::{Form, FormDescription, FormField, FormGroup, FormInput, FormLabel, FormProvider, FormSet};

#[derive(Debug, Clone, Default, Serialize, Deserialize, Validate)]
struct FormData {
    username: String,
    password: String,
}

#[component]
pub fn DemoForm() -> impl IntoView {
    let form = use_form::<FormData>();

    view! {
        <FormProvider form=form>
            <Form class="max-w-md">
                <FormSet>
                    <FormGroup>
                        <FormField field="username">
                            <FormLabel>Username</FormLabel>
                            <FormInput attr:placeholder="Max Wells" />
                            <FormDescription>Choose a unique username for your account.</FormDescription>
                        </FormField>
                        <FormField field="password">
                            <FormLabel>Password</FormLabel>
                            <FormDescription>Must be at least 8 characters long.</FormDescription>
                            <FormInput attr:r#type="password" attr:placeholder="********" />
                        </FormField>
                    </FormGroup>
                </FormSet>
            </Form>
        </FormProvider>
    }
}
