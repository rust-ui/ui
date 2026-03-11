use leptos::prelude::*;

use crate::ui::button::{Button, ButtonVariant};

#[component]
pub fn DemoButtonVariants() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 items-center @md:flex-row">
            <Button>Default</Button>

            <Button variant=ButtonVariant::Secondary>Secondary</Button>
            <Button variant=ButtonVariant::Outline>Outline</Button>
            <Button variant=ButtonVariant::Ghost>Ghost</Button>
            <Button variant=ButtonVariant::Destructive>Destructive</Button>
        </div>
    }
}
