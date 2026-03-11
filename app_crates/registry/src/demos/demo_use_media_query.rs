use leptos::prelude::*;

use crate::hooks::use_media_query::use_media_query;
use crate::ui::badge::{Badge, BadgeVariant};

#[component]
pub fn DemoUseMediaQuery() -> impl IntoView {
    let is_md = use_media_query("(min-width: 768px)");
    let is_lg = use_media_query("(min-width: 1024px)");

    let md_variant = Signal::derive(move || if is_md.get() { BadgeVariant::Default } else { BadgeVariant::Secondary });
    let lg_variant = Signal::derive(move || if is_lg.get() { BadgeVariant::Default } else { BadgeVariant::Secondary });

    view! {
        <div class="flex flex-col gap-4 items-center">
            <div class="flex gap-2 items-center">
                <span class="text-sm text-muted-foreground">"≥ 768px (md)"</span>
                <Badge variant=md_variant>{move || if is_md.get() { "matches" } else { "no match" }}</Badge>
            </div>
            <div class="flex gap-2 items-center">
                <span class="text-sm text-muted-foreground">"≥ 1024px (lg)"</span>
                <Badge variant=lg_variant>{move || if is_lg.get() { "matches" } else { "no match" }}</Badge>
            </div>
            <p class="text-xs text-muted-foreground">"Resize the window to see the signals update."</p>
        </div>
    }
}
