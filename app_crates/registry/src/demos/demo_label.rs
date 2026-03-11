use leptos::prelude::*;

use crate::ui::label::Label;

#[component]
pub fn DemoLabel() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <h2 class="text-2xl font-bold">Label Demo</h2>
            <div class="flex items-center space-x-2">
                // <Checkbox id="terms" checked=false />
                <Label html_for="terms">Accept terms and conditions</Label>
            </div>
        </div>
    }
}
