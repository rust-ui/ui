use leptos::prelude::*;

use crate::ui::switch::{Switch, SwitchLabel};

#[component]
pub fn DemoSwitch() -> impl IntoView {
    view! {
        <div class="flex gap-2">
            <Switch />
            <SwitchLabel>"Airplane"</SwitchLabel>
        </div>
    }
}
