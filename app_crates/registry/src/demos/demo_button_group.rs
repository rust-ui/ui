use leptos::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::button_group::ButtonGroup;

#[component]
pub fn DemoButtonGroup() -> impl IntoView {
    view! {
        <ButtonGroup>
            <Button variant=ButtonVariant::Outline>"First"</Button>
            <Button variant=ButtonVariant::Outline>"Second"</Button>
            <Button variant=ButtonVariant::Outline>"Third"</Button>
        </ButtonGroup>
    }
}
