use leptos::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::spinner::{Spinner, SpinnerCircle};

#[component]
pub fn DemoSpinnerRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <div class="flex gap-4 items-center">
                <Spinner />
                <SpinnerCircle />
                <span class="text-sm text-muted-foreground">"جاري التحميل..."</span>
            </div>
        </DirectionProvider>
    }
}
