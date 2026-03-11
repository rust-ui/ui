use leptos::prelude::*;

use crate::ui::progress::Progress;

#[component]
pub fn DemoProgress() -> impl IntoView {
    view! {
        <div class="w-full max-w-sm">
            <Progress value=60.0 />
        </div>
    }
}
