use icons::Search;
use leptos::prelude::*;

use crate::ui::input_group::{InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupInput};
use crate::ui::kbd::Kbd;

#[component]
pub fn DemoInputGroupKbd() -> impl IntoView {
    view! {
        <InputGroup class="max-w-sm">
            <InputGroupAddon>
                <Search />
            </InputGroupAddon>
            <InputGroupInput placeholder="Search..." />
            <InputGroupAddon align=InputGroupAddonAlign::InlineEnd>
                <Kbd>"⌘K"</Kbd>
            </InputGroupAddon>
        </InputGroup>
    }
}
