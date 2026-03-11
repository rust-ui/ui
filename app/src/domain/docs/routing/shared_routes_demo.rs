use std::sync::Arc;

use app_components::FooterLayout;
use app_config::{BreadcrumbItem, JsonLdArticle, JsonLdBreadcrumb, SeoMeta, SiteConfig};
use app_domain::markdown_config::RegistryEntry;
use app_domain::utils::{PARAM, ParamsUtils};
use app_routes::{ComponentsRoutes, HooksRoutes};
use leptos::either::Either;
use leptos::prelude::*;

use crate::__registry__::demos_sidenav::{get_all_arc_demos_for_components, get_all_arc_demos_for_hooks};
use crate::components::newsletter_signup::NewsletterSignup;
use crate::domain::docs::routing::docs_layout::DocsTocContext;
use crate::domain::markdown_ui::components::md_header::MdHeaderDemo;
use crate::domain::markdown_ui::components::md_prev_next::MdPrevNextDemos;
use crate::domain::markdown_ui::components::md_shared_with_toc::SharedDemoMdWithToc;
use crate::routes::page_not_found::PageNotFound;

// Gives back vec of ("Core", Arc::new(SIDENAV_COMPONENTS_CORE.to_vec())) for example.
fn get_all_arc_demos_for(route_path: &str) -> Vec<(&'static str, Arc<Vec<RegistryEntry>>)> {
    if route_path == ComponentsRoutes::base_url() {
        get_all_arc_demos_for_components()
    } else if route_path == HooksRoutes::base_url() {
        get_all_arc_demos_for_hooks()
    } else {
        vec![]
    }
}

#[component]
pub fn SharedRoutesDemo(route_path: &'static str) -> impl IntoView {
    let base_path = Arc::new(route_path.to_string());
    let params_demo_name = ParamsUtils::extract(PARAM::NAME);

    let all_arc_demos = get_all_arc_demos_for(route_path);

    let all_demos_combined =
        Arc::new(all_arc_demos.iter().flat_map(|(_, arc_demos)| arc_demos.iter()).copied().collect::<Vec<_>>());

    let all_demos = {
        let combined = Arc::clone(&all_demos_combined);
        move || combined.as_ref().clone()
    };

    let toc_context = expect_context::<DocsTocContext>();

    view! {
        <section data-name="__DocsSection" class="flex flex-col pt-4 mx-auto w-full min-h-screen md:px-4 max-w-[730px]">
            {move || {
                let params_demo_name = params_demo_name();
                let current_demo = all_demos().iter().find(|demo| demo.path_url == params_demo_name).copied();
                match current_demo {
                    Some(demo) => {
                        let title = format!("Leptos {} · Rust UI Components | {}", demo.title, SiteConfig::TITLE);
                        let canonical_url = format!("{}{}/{}", SiteConfig::BASE_URL, route_path, demo.path_url);
                        let meta_description = format!(
                            "Beautiful Rust UI {} component for Leptos applications. {}",
                            demo.title,
                            demo.description,
                        );
                        let (article_section, section_name) = if route_path == ComponentsRoutes::base_url() {
                            ("Components", "Components")
                        } else if route_path == HooksRoutes::base_url() {
                            ("Hooks", "Hooks")
                        } else {
                            ("Documentation", "Documentation")
                        };
                        let breadcrumbs = vec![
                            BreadcrumbItem {
                                name: "Home".to_string(),
                                url: Some(SiteConfig::BASE_URL.to_string()),
                            },
                            BreadcrumbItem {
                                name: section_name.to_string(),
                                url: Some(format!("{}{}", SiteConfig::BASE_URL, route_path)),
                            },
                            BreadcrumbItem {
                                name: demo.title.to_string(),
                                url: None,
                            },
                        ];
                        Either::Left(
                            // Determine article section based on route path

                            // Build breadcrumb trail
                            // Current page

                            view! {
                                <SeoMeta
                                    title=title
                                    description=meta_description
                                    canonical_url=canonical_url.clone()
                                    og_type="article".to_string()
                                />

                                <JsonLdArticle
                                    title=demo.title.to_string()
                                    description=demo.description.to_string()
                                    url=canonical_url
                                    keywords=demo.tags.iter().map(|t| t.to_string()).collect()
                                    article_section=article_section.to_string()
                                />

                                <JsonLdBreadcrumb breadcrumbs=breadcrumbs />

                                <MdHeaderDemo
                                    name=demo.title
                                    description=demo.description
                                    path_url=demo.path_url
                                    tags=demo.tags
                                    all_demos=Arc::clone(&all_demos_combined)
                                    current_demo=demo
                                    base_path=Arc::clone(&base_path)
                                />

                                <SharedDemoMdWithToc
                                    md_path=demo.path_md
                                    toc_signal=toc_context.toc_items.write_only()
                                />

                                <div class="mt-14 mb-6">
                                    <NewsletterSignup />
                                </div>

                                <MdPrevNextDemos
                                    all_demos=Arc::clone(&all_demos_combined)
                                    current_demo=demo
                                    base_path=Arc::clone(&base_path)
                                />

                                <FooterLayout />
                            },
                        )
                    }
                    None => {
                        Either::Right(

                            view! { <PageNotFound /> },
                        )
                    }
                }
            }}
        </section>
    }
}
