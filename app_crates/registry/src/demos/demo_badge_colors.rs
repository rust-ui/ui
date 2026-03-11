use leptos::prelude::*;

use crate::ui::badge::{Badge, BadgeVariant};

#[component]
pub fn DemoBadgeColors() -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-2">
            <Badge variant=BadgeVariant::Success>"Success"</Badge>
            <Badge variant=BadgeVariant::Warning>"Warning"</Badge>
            <Badge variant=BadgeVariant::Info>"Info"</Badge>
        </div>
    }
}
