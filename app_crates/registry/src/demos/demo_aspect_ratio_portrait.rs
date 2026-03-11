use leptos::prelude::*;

use crate::ui::aspect_ratio::AspectRatio;

#[component]
pub fn DemoAspectRatioPortrait() -> impl IntoView {
    view! {
        <div class="w-full max-w-[10rem]">
            <AspectRatio ratio=9.0_f64 / 16.0 class="rounded-lg bg-muted">
                <div class="bg-gradient-to-br from-rose-400 to-pink-600 rounded-lg size-full" />
            </AspectRatio>
        </div>
    }
}
