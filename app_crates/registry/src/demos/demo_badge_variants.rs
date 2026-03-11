use leptos::prelude::*;

use crate::ui::badge::{Badge, BadgeVariant};

#[component]
pub fn DemoBadgeVariants() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 items-center @md:flex-row">
            <Badge>Default</Badge>
            <Badge variant=BadgeVariant::Secondary>Secondary</Badge>
            <Badge variant=BadgeVariant::Destructive>Destructive</Badge>
            <Badge variant=BadgeVariant::Outline>Outline</Badge>
        </div>
    }
}
