use leptos::prelude::*;

use crate::ui::button::Button;
use crate::ui::field::{Field, FieldVariant};
use crate::ui::input::Input;

#[component]
pub fn DemoFieldInputInline() -> impl IntoView {
    view! {
        <Field variant=FieldVariant::Horizontal>
            <Input attr:r#type="search" placeholder="Search..." />
            <Button>"Search"</Button>
        </Field>
    }
}
