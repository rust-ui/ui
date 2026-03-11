use app_config::SiteConfig;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::Outlet;

use crate::components::navigation::header_docs::HeaderDocs;
use crate::domain::blocks::components::blocks_hero::BlocksHero;

#[component]
pub fn BlocksLayout() -> impl IntoView {
    let title = format!("Leptos UI Blocks & Components · Rust UI Library | {}", SiteConfig::TITLE);

    view! {
        <Title text=title />
        <HeaderDocs />

        <div data-name="__BlockLayout" class="container flex flex-col gap-20 pb-14">
            <BlocksHero />

            <Outlet />
        </div>
    }
}
