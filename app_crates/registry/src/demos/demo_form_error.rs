use leptos::prelude::*;

use crate::ui::form::{FormError, FormFieldWrapper, FormLabel};
use crate::ui::input::Input;

#[component]
pub fn DemoFormError() -> impl IntoView {
    view! {
        <div class="w-full max-w-md">
            <FormFieldWrapper attr:data-invalid="true">
                <FormLabel html_for="email">Email</FormLabel>
                <Input attr:id="email" attr:r#type="email" attr:aria-invalid="true" />
                <FormError>Enter a valid email address.</FormError>
            </FormFieldWrapper>
        </div>
    }
}
