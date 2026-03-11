use leptos::prelude::*;

use crate::ui::aspect_ratio::AspectRatio;

#[component]
pub fn DemoAspectRatio() -> impl IntoView {
    view! {
        <div class="w-full max-w-sm">
            <AspectRatio ratio=16.0_f64 / 9.0 class="rounded-lg bg-muted">
                <div class="bg-gradient-to-br from-violet-400 to-indigo-600 rounded-lg size-full" />
            </AspectRatio>
        </div>
    }
}
