use dioxus::prelude::*;
use icons::{ChevronsUpDown, LayoutTemplate, Sparkles};

use super::sidenav_routes::{ComponentsRoutes, DocsRoutes, HooksRoutes, SidenavRoutes};
use crate::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAction, DropdownMenuAlign, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem,
    DropdownMenuTrigger,
};

#[component]
pub fn SidenavRoutesSelector(current_section: DocsRoutes, sidenav_route: SidenavRoutes) -> Element {
    rsx! {
        DropdownMenu { align: DropdownMenuAlign::Center,
            DropdownMenuTrigger { class: "flex justify-between px-2 w-full h-12 bg-transparent border-0",
                div { class: "flex gap-2 items-center",
                    div { class: "flex justify-center items-center rounded-lg bg-primary text-primary-foreground aspect-square size-8",
                        match current_section {
                            DocsRoutes::Components => rsx! { LayoutTemplate {} },
                            DocsRoutes::Hooks => rsx! { Sparkles {} },
                        }
                    }
                    div { class: "grid flex-1 text-sm leading-tight text-left",
                        span { class: "font-medium", "Docs" }
                        span { class: "text-xs", "{current_section.to_title()}" }
                    }
                }
                ChevronsUpDown {}
            }
            DropdownMenuContent {
                DropdownMenuGroup {
                    for doc_route in [DocsRoutes::Components, DocsRoutes::Hooks] {
                        DropdownMenuItem {
                            DropdownMenuAction {
                                href: match doc_route {
                                    DocsRoutes::Components => ComponentsRoutes::base_url_with_sidenav(sidenav_route),
                                    DocsRoutes::Hooks => HooksRoutes::base_url_with_sidenav(sidenav_route),
                                },
                                "{doc_route.to_title()}"
                            }
                        }
                    }
                }
            }
        }
    }
}
