use leptos::prelude::*;

use crate::ui::field::{Field, FieldDescription, FieldGroup, FieldLabel, FieldSet};
use crate::ui::input::Input;

#[component]
pub fn DemoFieldInput() -> impl IntoView {
    view! {
        <FieldSet class="w-full max-w-xs">
            <FieldGroup>
                <Field>
                    <FieldLabel html_for="username">Username</FieldLabel>
                    <Input attr:id="username" placeholder="Max Leiter" />
                    <FieldDescription>Choose a unique username for your account.</FieldDescription>
                </Field>
                <Field>
                    <FieldLabel html_for="password">Password</FieldLabel>
                    <FieldDescription>Must be at least 8 characters long.</FieldDescription>
                    <Input attr:id="password" attr:r#type="password" placeholder="••••••••" />
                </Field>
            </FieldGroup>
        </FieldSet>
    }
}
