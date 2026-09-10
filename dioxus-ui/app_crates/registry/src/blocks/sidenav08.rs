use dioxus::prelude::*;

use super::sidenav_common::{self, SidenavPattern};
use super::sidenav_routes::{DocsRoutes, SidenavRoutes};
use crate::ui::sidenav::SidenavVariant;

#[component]
pub fn Sidenav08Sidebar(current_section: DocsRoutes, sidenav_route: SidenavRoutes) -> Element {
    sidenav_common::standard_sidebar(current_section, sidenav_route, SidenavPattern::Inset, SidenavVariant::Inset)
}
#[component]
pub fn Sidenav08Content(current_section: DocsRoutes, sidenav_route: SidenavRoutes) -> Element {
    rsx! { sidenav_common::SidenavStandardContent { current_section, sidenav_route, pattern: SidenavPattern::Inset } }
}
#[component]
pub fn Sidenav08MobileSheet(current_section: DocsRoutes, sidenav_route: SidenavRoutes) -> Element {
    sidenav_common::standard_mobile(current_section, sidenav_route, SidenavPattern::Inset, Default::default(), false)
}
#[component]
pub fn Sidenav08() -> Element {
    sidenav_common::EmptyLegacyBlock()
}
