


```rust
use dioxus::prelude::*;

use super::sidenav_common::{self, SidenavPattern};
use super::sidenav_routes::{DocsRoutes, SidenavRoutes};
use crate::components::ui::sheet::SheetDirection;
use crate::components::ui::sidenav::SidenavVariant;

#[component]
pub fn Sidenav04Sidebar(current_section: DocsRoutes, sidenav_route: SidenavRoutes) -> Element {
    sidenav_common::standard_sidebar(
        current_section,
        sidenav_route,
        SidenavPattern::Floating,
        SidenavVariant::Floating,
    )
}
#[component]
pub fn Sidenav04Content(current_section: DocsRoutes, sidenav_route: SidenavRoutes) -> Element {
    rsx! { sidenav_common::SidenavStandardContent { current_section, sidenav_route, pattern: SidenavPattern::Floating } }
}
#[component]
pub fn Sidenav04MobileSheet(current_section: DocsRoutes, sidenav_route: SidenavRoutes) -> Element {
    sidenav_common::standard_mobile(
        current_section,
        sidenav_route,
        SidenavPattern::Floating,
        SheetDirection::default(),
        false,
    )
}
#[component]
pub fn Sidenav04() -> Element {
    sidenav_common::EmptyLegacyBlock()
}
```