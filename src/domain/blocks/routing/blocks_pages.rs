use dioxus::prelude::*;

use crate::__registry__::all_blocks::{
    ALL_FAQ_BLOCKS, ALL_FOOTER_BLOCKS, ALL_HEADER_BLOCKS, ALL_INTEGRATION_BLOCKS, ALL_LOGIN_BLOCKS, ALL_SIDENAV_BLOCKS,
};
use crate::domain::blocks::components::block_viewer::BlockViewer;

#[component]
pub fn LoginBlocks() -> Element {
    rsx! {
        for block in ALL_LOGIN_BLOCKS {
            BlockViewer { block_entry: *block }
        }
    }
}

#[component]
pub fn SidenavBlocks() -> Element {
    rsx! {
        for block in ALL_SIDENAV_BLOCKS {
            BlockViewer { block_entry: *block }
        }
    }
}

#[component]
pub fn HeadersBlocks() -> Element {
    rsx! {
        for block in ALL_HEADER_BLOCKS {
            BlockViewer { block_entry: *block }
        }
    }
}

#[component]
pub fn FootersBlocks() -> Element {
    rsx! {
        for block in ALL_FOOTER_BLOCKS {
            BlockViewer { block_entry: *block }
        }
    }
}

#[component]
pub fn FaqBlocks() -> Element {
    rsx! {
        for block in ALL_FAQ_BLOCKS {
            BlockViewer { block_entry: *block }
        }
    }
}

#[component]
pub fn IntegrationsBlocks() -> Element {
    rsx! {
        for block in ALL_INTEGRATION_BLOCKS {
            BlockViewer { block_entry: *block }
        }
    }
}
