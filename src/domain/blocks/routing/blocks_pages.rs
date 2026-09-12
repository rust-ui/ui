use app_config::SeoMeta;
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
        }
        for block in ALL_INTEGRATION_BLOCKS {
            BlockViewer { block_entry: *block }
        }
    }
}
