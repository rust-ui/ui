use leptos::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::button_group::{ButtonGroup, ButtonGroupSeparator};

#[component]
pub fn DemoButtonGroupSeparator() -> impl IntoView {
    view! {
        <ButtonGroup>
            <Button variant=ButtonVariant::Secondary>"Copy"</Button>
            <ButtonGroupSeparator />
            <Button variant=ButtonVariant::Secondary>"Paste"</Button>
            <ButtonGroupSeparator />
            <Button variant=ButtonVariant::Secondary>"Cut"</Button>
        </ButtonGroup>
    }
}
