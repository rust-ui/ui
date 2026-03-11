use icons::Search;
use leptos::prelude::*;

use crate::ui::input_group::{InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupInput};
use crate::ui::kbd::Kbd;

#[component]
pub fn DemoKbdInputGroup() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6 w-full max-w-xs">
            <InputGroup>
                <InputGroupInput placeholder="Search..." />
                <InputGroupAddon>
                    <Search />
                </InputGroupAddon>
                <InputGroupAddon align=InputGroupAddonAlign::InlineEnd>
                    <Kbd>"⌘"</Kbd>
                    <Kbd>"K"</Kbd>
                </InputGroupAddon>
            </InputGroup>
        </div>
    }
}
