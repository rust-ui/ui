use leptos::prelude::*;

use crate::ui::button::Button;
use crate::ui::button_group::ButtonGroup;
use crate::ui::input::Input;

#[component]
pub fn DemoButtonGroupInput() -> impl IntoView {
    view! {
        <ButtonGroup>
            <Input placeholder="Enter a URL" class="w-64" />
            <Button>"Subscribe"</Button>
        </ButtonGroup>
    }
}
