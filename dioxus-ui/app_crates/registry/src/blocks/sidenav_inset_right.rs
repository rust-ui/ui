use dioxus::prelude::*;
use heck::ToTitleCase;
use icons::PanelLeft;

use super::sidenav_routes::{DocsRoutes, SidenavRoutes};
use crate::ui::breadcrumb::{
    Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator,
};
use crate::ui::separator::{Separator, SeparatorOrientation};
use crate::ui::sidenav::{SidenavInset, SidenavTrigger, SidenavVariant};

pub fn breadcrumb_from_path(path: &str, segment: &str) -> Vec<(String, String, bool)> {
    let parts: Vec<&str> = path.split('/').filter(|part| !part.is_empty()).collect();
    let Some(index) = parts.iter().position(|part| *part == segment) else { return Vec::new() };
    parts[index..]
        .iter()
        .enumerate()
        .map(|(offset, part)| {
            let end = index + offset;
            let href = format!("/{}", parts[..=end].join("/"));
            (part.to_title_case(), href, end == parts.len() - 1)
        })
        .collect()
}

#[component]
pub fn SidenavInsetRight(path: String, data_variant: Option<SidenavVariant>) -> Element {
    let breadcrumb_items = breadcrumb_from_path(&path, DocsRoutes::base_segment());
    let current_section =
        if path.contains(DocsRoutes::Components.as_ref()) { DocsRoutes::Components } else { DocsRoutes::Hooks };
    let sidenav_route = SidenavRoutes::from_path(&path);

    let data_variant = data_variant.map(|variant| match variant {
        SidenavVariant::Sidenav => "Sidenav".to_string(),
        SidenavVariant::Floating => "Floating".to_string(),
        SidenavVariant::Inset => "Inset".to_string(),
    });

    rsx! {
        SidenavInset { data_variant,
            header {
                class: "flex gap-2 items-center h-16 ease-linear shrink-0 transition-[width,height] group-has-data-[collapsible=icon]/sidenav-wrapper:h-12",
                div { class: "flex gap-2 items-center px-4",
                    match sidenav_route {
                        SidenavRoutes::Sidenav02 => rsx! { crate::blocks::sidenav02::Sidenav02MobileSheet { current_section, sidenav_route } },
                        SidenavRoutes::Sidenav03 => rsx! { crate::blocks::sidenav03::Sidenav03MobileSheet { current_section, sidenav_route } },
                        SidenavRoutes::Sidenav04 => rsx! { crate::blocks::sidenav04::Sidenav04MobileSheet { current_section, sidenav_route } },
                        SidenavRoutes::Sidenav05 => rsx! { crate::blocks::sidenav05::Sidenav05MobileSheet { current_section, sidenav_route } },
                        SidenavRoutes::Sidenav06 => rsx! { crate::blocks::sidenav06::Sidenav06MobileSheet { current_section, sidenav_route } },
                        SidenavRoutes::Sidenav07 => rsx! { crate::blocks::sidenav07::Sidenav07MobileSheet { current_section, sidenav_route } },
                        SidenavRoutes::Sidenav08 => rsx! { crate::blocks::sidenav08::Sidenav08MobileSheet { current_section, sidenav_route } },
                        SidenavRoutes::Sidenav09 => rsx! { crate::blocks::sidenav09::Sidenav09MobileSheet { current_section, sidenav_route } },
                        SidenavRoutes::Sidenav10 => rsx! { crate::blocks::sidenav10::Sidenav10MobileSheet { current_section, sidenav_route } },
                        SidenavRoutes::Sidenav11 => rsx! { crate::blocks::sidenav11::Sidenav11MobileSheet { current_section, sidenav_route } },
                        SidenavRoutes::Sidenav01 => rsx! { crate::blocks::sidenav01::Sidenav01MobileSheet { current_section, sidenav_route } },
                    }
                    div { class: "hidden md:block",
                        SidenavTrigger { PanelLeft {} span { class: "hidden", "Toggle Sidenav" } }
                    }
                    Separator { orientation: SeparatorOrientation::Vertical, class: "-ml-1 h-4" }
                    Breadcrumb {
                        BreadcrumbList {
                            for (idx, (name, href, is_last)) in breadcrumb_items.into_iter().enumerate() {
                                if idx > 0 { BreadcrumbSeparator {} }
                                BreadcrumbItem {
                                    if is_last { BreadcrumbPage { "{name}" } }
                                    else { BreadcrumbLink { href, "{name}" } }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "flex flex-col flex-1 gap-4 p-4 pt-0",
                div { class: "grid auto-rows-min gap-4 md:grid-cols-3",
                    div { class: "rounded-xl bg-muted/50 aspect-video" }
                    div { class: "rounded-xl bg-muted/50 aspect-video" }
                    div { class: "rounded-xl bg-muted/50 aspect-video" }
                }
                div { class: "flex-1 rounded-xl bg-muted/50 min-h-[100vh] md:min-h-min" }
            }
        }
    }
}
