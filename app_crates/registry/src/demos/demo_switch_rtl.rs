use leptos::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::switch::{Switch, SwitchLabel};

#[component]
pub fn DemoSwitchRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <div class="flex gap-2">
                <Switch />
                <SwitchLabel>"وضع الطائرة"</SwitchLabel>
            </div>
        </DirectionProvider>
    }
}
