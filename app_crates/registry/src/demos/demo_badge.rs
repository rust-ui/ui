use leptos::prelude::*;

use crate::ui::badge::Badge;

#[component]
pub fn DemoBadge() -> impl IntoView {
    view! { <Badge>"Default"</Badge> }
}
