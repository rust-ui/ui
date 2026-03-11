use leptos::prelude::*;

use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::kbd::Kbd;

#[component]
pub fn DemoKbdButton() -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-4 items-center">
            <Button variant=ButtonVariant::Outline size=ButtonSize::Sm class="pr-2">
                "Accept "
                <Kbd>"⏎"</Kbd>
            </Button>
            <Button variant=ButtonVariant::Outline size=ButtonSize::Sm class="pr-2">
                "Cancel "
                <Kbd>"Esc"</Kbd>
            </Button>
        </div>
    }
}
