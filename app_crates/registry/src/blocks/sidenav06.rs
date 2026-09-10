use dioxus::prelude::*;

use super::sidenav_common::{self, SidenavPattern};
use super::sidenav_routes::{DocsRoutes, SidenavRoutes};
use crate::ui::sheet::SheetDirection;
use crate::ui::sidenav::SidenavVariant;

#[component]
pub fn Sidenav06Sidebar(current_section: DocsRoutes, sidenav_route: SidenavRoutes) -> Element {
    sidenav_common::standard_sidebar(
        current_section,
        sidenav_route,
        SidenavPattern::Dropdown,
        SidenavVariant::default(),
    )
}
#[component]
pub fn Sidenav06Content(current_section: DocsRoutes, sidenav_route: SidenavRoutes) -> Element {
    rsx! { sidenav_common::SidenavStandardContent { current_section, sidenav_route, pattern: SidenavPattern::Dropdown } }
}
#[component]
pub fn Sidenav06MobileSheet(current_section: DocsRoutes, sidenav_route: SidenavRoutes) -> Element {
    sidenav_common::standard_mobile(
        current_section,
        sidenav_route,
        SidenavPattern::Dropdown,
        SheetDirection::default(),
        false,
    )
}
#[component]
pub fn Sidenav06() -> Element {
    sidenav_common::EmptyLegacyBlock()
}
