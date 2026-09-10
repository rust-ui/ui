use dioxus::prelude::*;
use registry::blocks::sidenav_inset_right::SidenavInsetRight;
use registry::blocks::sidenav_routes::{DocsRoutes, SidenavRoutes};
use registry::ui::sidenav::{SidenavVariant, SidenavWrapper};

use crate::Route;

#[component]
pub fn SidenavDemoLayout() -> Element {
    let path = use_route::<Route>().to_string();
    let sidenav_route = SidenavRoutes::from_path(&path);
    let current_section =
        if path.contains(DocsRoutes::Components.as_ref()) { DocsRoutes::Components } else { DocsRoutes::Hooks };

    rsx! {
        div { class: "bg-background",
            match sidenav_route {
                SidenavRoutes::Sidenav09 => rsx! {
                    SidenavWrapper { style: "--sidenav-width:18rem;--sidenav-width-icon:3rem;",
                        registry::blocks::sidenav09::Sidenav09Sidebar { current_section, sidenav_route }
                        Outlet::<Route> {}
                    }
                },
                SidenavRoutes::Sidenav11 => rsx! {
                    SidenavWrapper { style: "--sidenav-width:16rem;",
                        Outlet::<Route> {}
                        registry::blocks::sidenav11::Sidenav11Sidebar { current_section, sidenav_route }
                    }
                },
                other => rsx! {
                    SidenavWrapper { style: "--sidenav-width:16rem;",
                        SidenavDemoSidebar { sidenav_route: other, current_section }
                        Outlet::<Route> {}
                    }
                },
            }
        }
    }
}

#[component]
fn SidenavDemoSidebar(sidenav_route: SidenavRoutes, current_section: DocsRoutes) -> Element {
    match sidenav_route {
        SidenavRoutes::Sidenav02 => {
            rsx! { registry::blocks::sidenav02::Sidenav02Sidebar { current_section, sidenav_route } }
        }
        SidenavRoutes::Sidenav03 => {
            rsx! { registry::blocks::sidenav03::Sidenav03Sidebar { current_section, sidenav_route } }
        }
        SidenavRoutes::Sidenav04 => {
            rsx! { registry::blocks::sidenav04::Sidenav04Sidebar { current_section, sidenav_route } }
        }
        SidenavRoutes::Sidenav05 => {
            rsx! { registry::blocks::sidenav05::Sidenav05Sidebar { current_section, sidenav_route } }
        }
        SidenavRoutes::Sidenav06 => {
            rsx! { registry::blocks::sidenav06::Sidenav06Sidebar { current_section, sidenav_route } }
        }
        SidenavRoutes::Sidenav07 => {
            rsx! { registry::blocks::sidenav07::Sidenav07Sidebar { current_section, sidenav_route } }
        }
        SidenavRoutes::Sidenav08 => {
            rsx! { registry::blocks::sidenav08::Sidenav08Sidebar { current_section, sidenav_route } }
        }
        SidenavRoutes::Sidenav10 => {
            rsx! { registry::blocks::sidenav10::Sidenav10Sidebar { current_section, sidenav_route } }
        }
        _ => rsx! { registry::blocks::sidenav01::Sidenav01Sidebar { current_section, sidenav_route } },
    }
}

#[component]
pub fn SidenavInsetRightLayout() -> Element {
    let path = use_route::<Route>().to_string();
    let sidenav_route = SidenavRoutes::from_path(&path);
    let data_variant = match sidenav_route {
        SidenavRoutes::Sidenav08 => Some(SidenavVariant::Inset),
        _ => None,
    };

    rsx! {
        SidenavInsetRight { path, data_variant }
        Outlet::<Route> {}
    }
}
