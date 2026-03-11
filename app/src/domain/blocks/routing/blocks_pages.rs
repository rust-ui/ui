use app_config::SiteConfig;
use leptos::prelude::*;
use leptos_meta::Title;

use crate::domain::blocks::block_entry::BlockEntry;
use crate::domain::blocks::components::block_viewer::BlockViewer;

#[component]
fn BlocksPage(block_type: &'static str, get_blocks: fn() -> Vec<BlockEntry>) -> impl IntoView {
    let blocks = get_blocks();
    let title = format!("Leptos {} Blocks · Rust UI Components | {}", block_type, SiteConfig::TITLE);

    view! {
        <Title text=title />
        {blocks
            .into_iter()
            .map(|block_entry| {
                view! { <BlockViewer block_entry=block_entry /> }
            })
            .collect::<Vec<_>>()}
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn LoginBlocks() -> impl IntoView {
    view! { <BlocksPage block_type="Login" get_blocks=BlockEntry::get_login_blocks /> }
}

#[component]
pub fn SidenavBlocks() -> impl IntoView {
    view! { <BlocksPage block_type="Sidenav" get_blocks=BlockEntry::get_sidenav_blocks /> }
}

#[component]
pub fn HeadersBlocks() -> impl IntoView {
    view! { <BlocksPage block_type="Headers" get_blocks=BlockEntry::get_header_blocks /> }
}

#[component]
pub fn FootersBlocks() -> impl IntoView {
    view! { <BlocksPage block_type="Footers" get_blocks=BlockEntry::get_footer_blocks /> }
}

#[component]
pub fn FaqBlocks() -> impl IntoView {
    view! { <BlocksPage block_type="Footers" get_blocks=BlockEntry::get_faq_blocks /> }
}

#[component]
pub fn IntegrationsBlocks() -> impl IntoView {
    view! { <BlocksPage block_type="Integrations" get_blocks=BlockEntry::get_integration_blocks /> }
}
