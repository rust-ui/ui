use app_config::SiteConfig;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::Outlet;
use leptos_router::hooks::use_location;

use crate::components::navigation::header_docs::HeaderDocs;
use crate::domain::blocks::components::blocks_hero::BlocksHero;
use crate::utils::page_transition::{PAGE_OUTLET, retrigger_page_fade};

#[component]
pub fn BlocksLayout() -> impl IntoView {
    let title = format!("Leptos UI Blocks & Components · Rust UI Library | {}", SiteConfig::TITLE);
    let location = use_location();

    Effect::new(move |prev: Option<()>| {
        let _ = location.pathname.get();
        if prev.is_some() {
            retrigger_page_fade();
        }
    });

    view! {
        <Title text=title />
        <HeaderDocs />

        <div data-name="__BlockLayout" class="container flex flex-col gap-20 pb-14">
            <BlocksHero />

            <div id=PAGE_OUTLET class="flex flex-col gap-20 page__fade">
                <Outlet />
            </div>
        </div>
    }
}
