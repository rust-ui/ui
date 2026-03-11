use leptos::prelude::*;

use crate::ui::field::{Field, FieldLabel};
use crate::ui::progress::Progress;

#[component]
pub fn DemoProgressLabel() -> impl IntoView {
    view! {
        <Field class="w-full max-w-sm">
            <FieldLabel html_for="progress-upload">
                <span>"Upload progress"</span>
                <span class="ml-auto">"66%"</span>
            </FieldLabel>
            <Progress value=66.0 />
        </Field>
    }
}
