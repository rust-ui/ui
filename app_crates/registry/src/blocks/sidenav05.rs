use dioxus::prelude::*;

use super::sidenav_common::{self, SidenavPattern};
use super::sidenav_routes::{DocsRoutes, SidenavRoutes};

#[component]
pub fn Sidenav05Sidebar(current_section: DocsRoutes, sidenav_route: SidenavRoutes) -> Element {
    sidenav_common::standard_sidebar(
        current_section,
        sidenav_route,
        SidenavPattern::CollapsibleSubmenus,
        Default::default(),
    )
}
#[component]
pub fn Sidenav05Content(current_section: DocsRoutes, sidenav_route: SidenavRoutes) -> Element {
    rsx! { sidenav_common::SidenavStandardContent { current_section, sidenav_route, pattern: SidenavPattern::CollapsibleSubmenus } }
}
#[component]
pub fn Sidenav05MobileSheet(current_section: DocsRoutes, sidenav_route: SidenavRoutes) -> Element {
    sidenav_common::standard_mobile(
        current_section,
        sidenav_route,
        SidenavPattern::CollapsibleSubmenus,
        Default::default(),
        false,
    )
}
#[component]
pub fn Sidenav05() -> Element {
    sidenav_common::EmptyLegacyBlock()
}
