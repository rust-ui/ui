use leptos::prelude::*;

use crate::ui::badge::Badge;

#[component]
pub fn DemoBadgeCustom() -> impl IntoView {
    view! {
        <div class="flex gap-2 items-center">
            <Badge class="bg-sky-500">Cutstom</Badge>
        </div>
    }
}
