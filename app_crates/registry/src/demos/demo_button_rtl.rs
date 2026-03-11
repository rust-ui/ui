use leptos::prelude::*;

use crate::ui::button::Button;
use crate::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoButtonRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <Button>"زر"</Button>
        </DirectionProvider>
    }
}
