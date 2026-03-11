use leptos::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::progress::Progress;

#[component]
pub fn DemoProgressRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-sm">
            <div class="flex flex-col gap-4 w-full max-w-sm">
                <p class="text-sm text-muted-foreground">"تقدم التحميل"</p>
                <Progress value=60.0 />
            </div>
        </DirectionProvider>
    }
}
