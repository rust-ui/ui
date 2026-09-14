use app_config::{BreadcrumbItem, JsonLdBreadcrumb, SeoMeta, SiteConfig};
use dioxus::prelude::*;

use crate::__registry__::all_blocks::{
    ALL_FAQ_BLOCKS, ALL_FOOTER_BLOCKS, ALL_HEADER_BLOCKS, ALL_INTEGRATION_BLOCKS, ALL_LOGIN_BLOCKS, ALL_SIDENAV_BLOCKS,
};
use crate::domain::blocks::components::block_viewer::BlockViewer;

#[component]
pub fn LoginBlocks() -> Element {
    rsx! {
        SeoMeta {
            title: "Login Blocks · Rust UI Component Library | Rust/UI".to_string(),
            description: "Ready-made login page blocks built with Rust/UI components for Dioxus applications."
                .to_string(),
            canonical_url: format!("{}/blocks/login", SiteConfig::BASE_URL),
        }
        JsonLdBreadcrumb {
            breadcrumbs: vec![
                BreadcrumbItem { name: "Home".to_string(), url: Some(SiteConfig::BASE_URL.to_string()) },
                BreadcrumbItem { name: "Login Blocks".to_string(), url: None },
            ],
        }
        for block in ALL_LOGIN_BLOCKS {
            BlockViewer { block_entry: *block }
        }
    }
}

#[component]
pub fn SidenavBlocks() -> Element {
    rsx! {
        SeoMeta {
            title: "Sidenav Blocks · Rust UI Component Library | Rust/UI".to_string(),
            description: "Ready-made side navigation blocks built with Rust/UI components for Dioxus applications."
                .to_string(),
            canonical_url: format!("{}/blocks/sidenav", SiteConfig::BASE_URL),
        }
        JsonLdBreadcrumb {
            breadcrumbs: vec![
                BreadcrumbItem { name: "Home".to_string(), url: Some(SiteConfig::BASE_URL.to_string()) },
                BreadcrumbItem { name: "Sidenav Blocks".to_string(), url: None },
            ],
        }
        for block in ALL_SIDENAV_BLOCKS {
            BlockViewer { block_entry: *block }
        }
    }
}

#[component]
pub fn HeadersBlocks() -> Element {
    rsx! {
        SeoMeta {
            title: "Header Blocks · Rust UI Component Library | Rust/UI".to_string(),
            description: "Ready-made header blocks built with Rust/UI components for Dioxus applications."
                .to_string(),
            canonical_url: format!("{}/blocks/headers", SiteConfig::BASE_URL),
        }
        JsonLdBreadcrumb {
            breadcrumbs: vec![
                BreadcrumbItem { name: "Home".to_string(), url: Some(SiteConfig::BASE_URL.to_string()) },
                BreadcrumbItem { name: "Header Blocks".to_string(), url: None },
            ],
        }
        for block in ALL_HEADER_BLOCKS {
            BlockViewer { block_entry: *block }
        }
    }
}

#[component]
pub fn FootersBlocks() -> Element {
    rsx! {
        SeoMeta {
            title: "Footer Blocks · Rust UI Component Library | Rust/UI".to_string(),
            description: "Ready-made footer blocks built with Rust/UI components for Dioxus applications."
                .to_string(),
            canonical_url: format!("{}/blocks/footers", SiteConfig::BASE_URL),
        }
        JsonLdBreadcrumb {
            breadcrumbs: vec![
                BreadcrumbItem { name: "Home".to_string(), url: Some(SiteConfig::BASE_URL.to_string()) },
                BreadcrumbItem { name: "Footer Blocks".to_string(), url: None },
            ],
        }
        for block in ALL_FOOTER_BLOCKS {
            BlockViewer { block_entry: *block }
        }
    }
}

#[component]
pub fn FaqBlocks() -> Element {
    rsx! {
        SeoMeta {
            title: "FAQ Blocks · Rust UI Component Library | Rust/UI".to_string(),
            description: "Ready-made FAQ section blocks built with Rust/UI components for Dioxus applications."
                .to_string(),
            canonical_url: format!("{}/blocks/faq", SiteConfig::BASE_URL),
        }
        JsonLdBreadcrumb {
            breadcrumbs: vec![
                BreadcrumbItem { name: "Home".to_string(), url: Some(SiteConfig::BASE_URL.to_string()) },
                BreadcrumbItem { name: "FAQ Blocks".to_string(), url: None },
            ],
        }
        for block in ALL_FAQ_BLOCKS {
            BlockViewer { block_entry: *block }
        }
    }
}

#[component]
pub fn IntegrationsBlocks() -> Element {
    rsx! {
        SeoMeta {
            title: "Integrations Blocks · Rust UI Component Library | Rust/UI".to_string(),
            description: "Ready-made integrations section blocks built with Rust/UI components for Dioxus applications."
                .to_string(),
            canonical_url: format!("{}/blocks/integrations", SiteConfig::BASE_URL),
        }
        JsonLdBreadcrumb {
            breadcrumbs: vec![
                BreadcrumbItem { name: "Home".to_string(), url: Some(SiteConfig::BASE_URL.to_string()) },
                BreadcrumbItem { name: "Integrations Blocks".to_string(), url: None },
            ],
        }
        for block in ALL_INTEGRATION_BLOCKS {
            BlockViewer { block_entry: *block }
        }
    }
}
