use app_components::LogoHomeLink;
use app_domain::constants::RoutePaths;
use app_routes::{ComponentsRoutes, HooksRoutes};
use leptos::prelude::*;
use registry::ui::link::{Link, PathMatchType};

#[component]
pub fn NavDesktop() -> impl IntoView {
    view! {
        <div class="hidden gap-0 items-center md:flex">

            <LogoHomeLink />

            <Link
                href=RoutePaths::DOCS_INTRODUCTION
                class="inline-flex items-center py-1.5 px-2.5 text-sm rounded-md hover:bg-accent"
                match_type=PathMatchType::MatchAny(
                    vec![
                        "/docs/components/introduction".to_string(),
                        "/docs/components/installation".to_string(),
                        "/docs/components/cli".to_string(),
                        "/docs/components/icons".to_string(),
                        "/docs/components/figma".to_string(),
                        "/docs/components/changelog".to_string(),
                    ],
                )
            >
                Docs
            </Link>
            <Link
                href=ComponentsRoutes::Button.to_route()
                class="inline-flex items-center py-1.5 px-2.5 text-sm rounded-md hover:bg-accent"
                match_type=PathMatchType::StartsWithExcept(
                    ComponentsRoutes::base_url().to_string(),
                    vec![
                        "/docs/components/introduction".to_string(),
                        "/docs/components/installation".to_string(),
                        "/docs/components/cli".to_string(),
                        "/docs/components/icons".to_string(),
                        "/docs/components/figma".to_string(),
                        "/docs/components/changelog".to_string(),
                    ],
                )
            >
                "Components"
            </Link>
            <Link
                href=HooksRoutes::UseCopyClipboard.to_route()
                class="inline-flex items-center py-1.5 px-2.5 text-sm rounded-md hover:bg-accent"
            >
                Hooks
            </Link>
            <Link
                href=RoutePaths::ICONS
                class="inline-flex items-center py-1.5 px-2.5 text-sm rounded-md hover:bg-accent"
            >
                Icons
            </Link>
            <Link
                href=RoutePaths::BLOCKS
                class="inline-flex items-center py-1.5 px-2.5 text-sm rounded-md hover:bg-accent"
            >
                Blocks
            </Link>
            // TODO. 🚑 Shortfix to force reload the page, small issue with SPA when navigate from Blocks to Charts using this Link. Some Charts are rendered twice in the UI...
            <Link
                href=RoutePaths::CHARTS
                force_reload=true
                class="inline-flex items-center py-1.5 px-2.5 text-sm rounded-md hover:bg-accent"
            >
                Charts
            </Link>
            <Link
                href=RoutePaths::CREATE
                class="inline-flex items-center py-1.5 px-2.5 text-sm rounded-md hover:bg-accent"
            >
                Create
            </Link>
        </div>
    }
}
