use dioxus::prelude::*;

use crate::ui::breadcrumb::{
    Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator,
};
use crate::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoBreadcrumbRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "w-full max-w-lg",
            Breadcrumb {
                BreadcrumbList {
                    BreadcrumbItem {
                        BreadcrumbLink { href: "/", "الرئيسية" }
                    }
                    BreadcrumbSeparator {}
                    BreadcrumbItem {
                        span { class: "flex h-9 w-9 items-center justify-center",
                            span { class: "text-muted-foreground text-sm", "…" }
                        }
                    }
                    BreadcrumbSeparator {}
                    BreadcrumbItem {
                        BreadcrumbLink { href: "/demo-components/button", "المكونات" }
                    }
                    BreadcrumbSeparator {}
                    BreadcrumbItem {
                        BreadcrumbPage { "التوثيق" }
                    }
                }
            }
        }
    }
}
