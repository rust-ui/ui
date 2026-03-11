use leptos::prelude::*;

use crate::ui::input_group::{InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupInput, InputGroupText};
use crate::ui::spinner::{Spinner, SpinnerCircle};

#[component]
pub fn DemoInputGroupSpinner() -> impl IntoView {
    view! {
        <div class="grid gap-4 w-full max-w-sm">
            <InputGroup>
                <InputGroupInput placeholder="Searching..." />
                <InputGroupAddon align=InputGroupAddonAlign::InlineEnd>
                    <Spinner />
                </InputGroupAddon>
            </InputGroup>

            <InputGroup>
                <InputGroupAddon>
                    <SpinnerCircle />
                </InputGroupAddon>
                <InputGroupInput placeholder="Processing..." />
            </InputGroup>

            <InputGroup>
                <InputGroupInput placeholder="Saving changes..." />
                <InputGroupAddon align=InputGroupAddonAlign::InlineEnd>
                    <InputGroupText>"Saving..."</InputGroupText>
                    <Spinner />
                </InputGroupAddon>
            </InputGroup>
        </div>
    }
}
