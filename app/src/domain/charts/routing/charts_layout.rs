use app_components::FooterLayout;
use app_config::SiteConfig;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::Outlet;

use crate::components::navigation::header_docs::HeaderDocs;
use crate::domain::charts::components::charts_hero::ChartsHero;

#[component]
pub fn ChartsLayout() -> impl IntoView {
    let title = format!("Leptos Charts & Graphs · Rust UI Components | {}", SiteConfig::TITLE);

    view! {
        <Title text=title />
        // Load ApexCharts only on chart pages for performance optimization
        <script id="apexcharts-cdn" defer src="/cdn/apexcharts.5.3.6.min.js"></script>
        <script id="chart-init-script" defer src="/coming_soon/chart_init.js"></script>

        <HeaderDocs />

        <div data-name="__ChartsLayout" class="container flex flex-col gap-10">
            <ChartsHero />

            <Outlet />
        </div>

        <FooterLayout />
    }
}
