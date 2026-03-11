use leptos::prelude::*;

use crate::ui::badge::Badge;
use crate::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoBadgeRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <Badge>"افتراضي"</Badge>
        </DirectionProvider>
    }
}
