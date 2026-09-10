use dioxus::prelude::*;

use super::sidenav_common::{self, SidenavPattern};
use super::sidenav_routes::{DocsRoutes, SidenavRoutes};
use crate::ui::sheet::SheetDirection;
use crate::ui::sidenav::SidenavVariant;

#[component]
pub fn Sidenav07Sidebar(current_section: DocsRoutes, sidenav_route: SidenavRoutes) -> Element {
    sidenav_common::standard_sidebar(
        current_section,
        sidenav_route,
        SidenavPattern::Icons,
        SidenavVariant::default(),
    )
}
#[component]
pub fn Sidenav07Content(current_section: DocsRoutes, sidenav_route: SidenavRoutes) -> Element {
    rsx! { sidenav_common::SidenavStandardContent { current_section, sidenav_route, pattern: SidenavPattern::Icons } }
}
#[component]
pub fn Sidenav07MobileSheet(current_section: DocsRoutes, sidenav_route: SidenavRoutes) -> Element {
    sidenav_common::standard_mobile(
        current_section,
        sidenav_route,
        SidenavPattern::Icons,
        SheetDirection::default(),
        false,
    )
}
#[component]
pub fn Sidenav07() -> Element {
    sidenav_common::EmptyLegacyBlock()
}
