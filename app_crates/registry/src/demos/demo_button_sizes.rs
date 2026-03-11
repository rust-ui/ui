use leptos::prelude::*;

use crate::ui::button::{Button, ButtonSize};

#[component]
pub fn DemoButtonSizes() -> impl IntoView {
    view! {
        <div class="flex gap-4">
            <Button size=ButtonSize::Sm>Small</Button>
            <Button>Default</Button>
            <Button size=ButtonSize::Lg>Large</Button>
        </div>
    }
}
