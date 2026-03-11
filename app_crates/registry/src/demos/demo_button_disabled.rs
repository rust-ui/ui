use leptos::prelude::*;

use crate::ui::button::Button;

#[component]
pub fn DemoButtonDisabled() -> impl IntoView {
    view! {
        <div class="flex gap-4">
            <Button>"Normal"</Button>
            <Button attr:disabled=true>"Disabled"</Button>
        </div>
    }
}
