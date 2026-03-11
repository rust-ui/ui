use leptos::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::textarea::Textarea;

#[component]
pub fn DemoTextareaRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-sm">
            <div>
                <Textarea attr:placeholder="اكتب رسالتك هنا." />
            </div>
        </DirectionProvider>
    }
}
