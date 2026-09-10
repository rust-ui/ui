use app_config::{BreadcrumbItem, JsonLdArticle, JsonLdBreadcrumb, SeoMeta, SiteConfig};
use dioxus::prelude::*;
use icons::{ChevronLeft, ChevronRight};

use crate::__registry__::static_md_registry::{MyMd, docs_component_prev_next, find_docs_component_entry};
use crate::components::doc_header::DocHeader;
use crate::components::footer_layout::FooterLayout;
use crate::components::newsletter_signup::NewsletterSignup;
use crate::registry::types::RegistryEntry;
use crate::routes::page_not_found::PageNotFound;

#[component]
pub fn ComponentPage(name: String) -> Element {
    let entry = find_docs_component_entry(&name);
    let (prev, next) = docs_component_prev_next(&name);

    rsx! {
        div { class: "flex flex-col pt-4 mx-auto w-full min-h-screen px-3 md:px-4 max-w-[730px]",
            match entry {
                None => rsx! {
                    PageNotFound { segments: vec!["docs".into(), "components".into(), name.clone()] }
                },
                Some(e) => {
                    let page_title = format!(
                        "Dioxus {} · Rust UI Components | {}",
                        e.title(),
                        SiteConfig::TITLE,
                    );
                    let canonical_url = format!("{}/docs/components/{}", SiteConfig::BASE_URL, e.slug);
                    let meta_description = format!(
                        "Beautiful Rust UI {} component for Dioxus applications. {}",
                        e.title(),
                        e.description(),
                    );
                    let breadcrumbs = vec![
                        BreadcrumbItem {
                            name: "Home".to_string(),
                            url: Some(SiteConfig::BASE_URL.to_string()),
                        },
                        BreadcrumbItem {
                            name: "Components".to_string(),
                            url: Some(format!("{}/docs/components", SiteConfig::BASE_URL)),
                        },
                        BreadcrumbItem {
                            name: e.title(),
                            url: None,
                        },
                    ];
                    rsx! {
                        // Key the whole arm on the slug: dioxus reuses `ComponentPage`
                        // across `/docs/components/:name` navigations (same slot, new
                        // prop), so without this the `document::*` nodes inside
                        // `SeoMeta` get diffed in place and log "Changing the props of
                        // `Meta {}` is not supported". Keying remounts the subtree on
                        // navigation, matching how leptos recreates the route view.
                        SeoMeta {
                            key: "{e.slug}",
                            title: page_title,
                            description: meta_description,
                            canonical_url: canonical_url.clone(),
                            og_type: "article".to_string(),
                        }

                        JsonLdArticle {
                            title: e.title(),
                            description: e.description(),
                            url: canonical_url,
                            keywords: e.tags.iter().map(|t| t.to_string()).collect(),
                            article_section: "Components".to_string(),
                        }

                        JsonLdBreadcrumb { breadcrumbs }

                        DocHeader {
                            title: e.title(),
                            description: e.description(),
                            tags: e.tags.to_vec(),
                            raw: e.raw,
                            slug: e.slug,
                            section_label: "Components".to_string(),
                            base_path: "/docs/components".to_string(),
                            prev,
                            next,
                        }
                        MyMd { raw: e.raw }
                        div { class: "mt-14 mb-6", NewsletterSignup {} }
                        DocBottomNav { prev, next }
                        FooterLayout {}
                    }
                }
            }
        }
    }
}

#[component]
fn DocBottomNav(prev: Option<&'static RegistryEntry>, next: Option<&'static RegistryEntry>) -> Element {
    rsx! {
        div { class: "flex justify-between items-center mt-8",
            if let Some(p) = prev {
                a {
                    href: "/docs/components/{p.slug}",
                    class: "py-0 px-2 h-8 inline-flex justify-center items-center text-sm font-medium whitespace-nowrap rounded-md transition-colors w-fit focus-visible:outline-hidden focus-visible:ring-1 focus-visible:ring-ring border bg-background border-input hover:bg-accent hover:text-accent-foreground z-50",
                    ChevronLeft {}
                    span { "{p.title()}" }
                }
            } else {
                div {}
            }
            if let Some(n) = next {
                a {
                    href: "/docs/components/{n.slug}",
                    class: "py-0 px-2 h-8 inline-flex justify-center items-center text-sm font-medium whitespace-nowrap rounded-md transition-colors w-fit focus-visible:outline-hidden focus-visible:ring-1 focus-visible:ring-ring border bg-background border-input hover:bg-accent hover:text-accent-foreground z-50",
                    span { "{n.title()}" }
                    ChevronRight {}
                }
            } else {
                div {}
            }
        }
    }
}
