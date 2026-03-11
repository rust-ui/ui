use leptos::prelude::*;

use crate::ui::button::{Button, ButtonVariant};

#[component]
pub fn DemoButtonHref() -> impl IntoView {
    view! {
        <div class="flex gap-10 p-4 rounded-lg border">
            <Button href="/dashboard">"Go to Dashboard"</Button>
            <Button href="/dashboard" variant=ButtonVariant::Ghost class="border-2 border-dashed">
                "Go to Dashboard (custom)"
            </Button>
            <Button href="https://ever-ui.com/" attr:rel="noopener" attr:target="_blank">
                "External Link"
            </Button>
        </div>
    }
}
