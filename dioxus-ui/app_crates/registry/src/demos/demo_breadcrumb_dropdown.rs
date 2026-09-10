use dioxus::prelude::*;

use crate::ui::breadcrumb::{
    Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator,
};
use crate::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLink, DropdownMenuTrigger,
};

#[component]
pub fn DemoBreadcrumbDropdown() -> Element {
    rsx! {
        Breadcrumb {
            BreadcrumbList {
                BreadcrumbItem {
                    BreadcrumbLink { href: "/", "Home" }
                }

                BreadcrumbSeparator {}

                BreadcrumbItem {
                    DropdownMenu {
                        DropdownMenuTrigger {
                            BreadcrumbEllipsis {}
                        }
                        DropdownMenuContent {
                            DropdownMenuItem {
                                DropdownMenuLink { href: "/docs", "Documentation" }
                            }
                            DropdownMenuItem {
                                DropdownMenuLink { href: "/docs/components", "Components" }
                            }
                            DropdownMenuItem {
                                DropdownMenuLink { href: "/docs/components/button", "Button" }
                            }
                        }
                    }
                }

                BreadcrumbSeparator {}

                BreadcrumbItem {
                    BreadcrumbLink { href: "/docs/components/breadcrumb", "Breadcrumb" }
                }

                BreadcrumbSeparator {}

                BreadcrumbItem {
                    BreadcrumbPage { "Dropdown" }
                }
            }
        }
    }
}
