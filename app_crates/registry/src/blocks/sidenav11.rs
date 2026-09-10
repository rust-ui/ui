use dioxus::prelude::*;

use super::sidenav_common::{self, SidenavPattern};
use super::sidenav_routes::{DocsRoutes, SidenavRoutes};
use crate::ui::sidenav::{Sidenav, SidenavSide};

#[component]
pub fn Sidenav11Sidebar(current_section: DocsRoutes, sidenav_route: SidenavRoutes) -> Element {
    rsx! { Sidenav { data_side: SidenavSide::Right,
        sidenav_common::SidenavStandardContent { current_section, sidenav_route, pattern: SidenavPattern::Right }
    } }
}
#[component]
pub fn Sidenav11Content(current_section: DocsRoutes, sidenav_route: SidenavRoutes) -> Element {
    rsx! { sidenav_common::SidenavStandardContent { current_section, sidenav_route, pattern: SidenavPattern::Right } }
}
#[component]
pub fn Sidenav11MobileSheet(current_section: DocsRoutes, sidenav_route: SidenavRoutes) -> Element {
    sidenav_common::standard_mobile(
        current_section,
        sidenav_route,
        SidenavPattern::Right,
        crate::ui::sheet::SheetDirection::Right,
        true,
    )
}
#[component]
pub fn Sidenav11() -> Element {
    sidenav_common::EmptyLegacyBlock()
}
