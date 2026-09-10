use dioxus::prelude::*;
use icons::{Component, Layers};

use super::sidenav_common::{self, SidenavPattern};
use super::sidenav_routes::{DocsRoutes, SidenavRoutes};
use crate::demos::demo_dropdown_menu_user_icon::DemoDropdownMenuUserIcon;
use crate::ui::sidenav::{Sidenav, SidenavCollapsible, SidenavMenu, SidenavMenuButton, SidenavMenuItem};

#[component]
pub fn Sidenav09Sidebar(current_section: DocsRoutes, sidenav_route: SidenavRoutes) -> Element {
    rsx! {
        Sidenav { data_collapsible: SidenavCollapsible::Icon, class: "overflow-hidden *:data-[sidenav=Sidenav]:flex-row",
            Sidenav { data_collapsible: SidenavCollapsible::None, class: "border-r w-[calc(var(--sidenav-width-icon)+1px)]!",
                SidenavMenu {
                    SidenavMenuItem { SidenavMenuButton { href: format!("/{}/docs/components", sidenav_route.to_route()), Component {} } }
                    SidenavMenuItem { SidenavMenuButton { href: format!("/{}/docs/hooks", sidenav_route.to_route()), Layers {} } }
                }
                crate::ui::sidenav::SidenavFooter { DemoDropdownMenuUserIcon {} }
            }
            Sidenav { data_collapsible: SidenavCollapsible::None, class: "hidden flex-1 md:flex",
                sidenav_common::SidenavStandardContent { current_section, sidenav_route, pattern: SidenavPattern::CollapsibleSubmenus }
            }
        }
    }
}
#[component]
pub fn Sidenav09Content(current_section: DocsRoutes, sidenav_route: SidenavRoutes) -> Element {
    rsx! { sidenav_common::SidenavStandardContent { current_section, sidenav_route, pattern: SidenavPattern::CollapsibleSubmenus } }
}
#[component]
pub fn Sidenav09MobileSheet(current_section: DocsRoutes, sidenav_route: SidenavRoutes) -> Element {
    sidenav_common::standard_mobile(
        current_section,
        sidenav_route,
        SidenavPattern::CollapsibleSubmenus,
        Default::default(),
        false,
    )
}
#[component]
pub fn Sidenav09() -> Element {
    sidenav_common::EmptyLegacyBlock()
}
