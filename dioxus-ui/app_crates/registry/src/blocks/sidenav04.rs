use dioxus::prelude::*;

use super::sidenav_common::{self, SidenavPattern};
use super::sidenav_routes::{DocsRoutes, SidenavRoutes};
use crate::ui::sidenav::SidenavVariant;

#[component]
pub fn Sidenav04Sidebar(current_section: DocsRoutes, sidenav_route: SidenavRoutes) -> Element {
    sidenav_common::standard_sidebar(current_section, sidenav_route, SidenavPattern::Floating, SidenavVariant::Floating)
}
#[component]
pub fn Sidenav04Content(current_section: DocsRoutes, sidenav_route: SidenavRoutes) -> Element {
    rsx! { sidenav_common::SidenavStandardContent { current_section, sidenav_route, pattern: SidenavPattern::Floating } }
}
#[component]
pub fn Sidenav04MobileSheet(current_section: DocsRoutes, sidenav_route: SidenavRoutes) -> Element {
    sidenav_common::standard_mobile(current_section, sidenav_route, SidenavPattern::Floating, Default::default(), false)
}
#[component]
pub fn Sidenav04() -> Element {
    sidenav_common::EmptyLegacyBlock()
}
