use leptos::prelude::*;

use crate::ui::input_group::{InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupInput, InputGroupText};

#[component]
pub fn DemoInputGroupText() -> impl IntoView {
    view! {
        <div class="grid gap-6 w-full max-w-sm">
            <InputGroup>
                <InputGroupAddon>
                    <InputGroupText>$</InputGroupText>
                </InputGroupAddon>
                <InputGroupInput placeholder="0.00" />
                <InputGroupAddon align=InputGroupAddonAlign::InlineEnd>
                    <InputGroupText>USD</InputGroupText>
                </InputGroupAddon>
            </InputGroup>

            <InputGroup>
                <InputGroupAddon>
                    <InputGroupText>{"https://"}</InputGroupText>
                </InputGroupAddon>
                <InputGroupInput placeholder="example.com" class="!pl-0.5" />
                <InputGroupAddon align=InputGroupAddonAlign::InlineEnd>
                    <InputGroupText>{".com"}</InputGroupText>
                </InputGroupAddon>
            </InputGroup>

            <InputGroup>
                <InputGroupInput placeholder="Enter your username" />
                <InputGroupAddon align=InputGroupAddonAlign::InlineEnd>
                    <InputGroupText>@company.com</InputGroupText>
                </InputGroupAddon>
            </InputGroup>
        </div>
    }
}
