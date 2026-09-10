use dioxus::prelude::*;

use crate::ui::collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use crate::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoCollapsibleRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "max-w-fit",
            Collapsible { class: "flex flex-col gap-2 w-[350px]",
                div { class: "flex gap-4 justify-between items-center px-4",
                    h4 { class: "text-sm font-semibold", "طلب رقم #٤١٨٩" }
                    CollapsibleTrigger { class: "inline-flex justify-center items-center rounded-md transition-colors size-8 hover:bg-accent hover:text-accent-foreground",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            class: "size-4",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "m7 15 5 5 5-5" }
                            path { d: "m7 9 5-5 5 5" }
                        }
                        span { class: "sr-only", "تبديل التفاصيل" }
                    }
                }
                div { class: "flex justify-between items-center py-2 px-4 text-sm rounded-md border",
                    span { class: "text-muted-foreground", "الحالة" }
                    span { class: "font-medium", "تم الشحن" }
                }
                CollapsibleContent { class: "flex flex-col gap-2",
                    div { class: "py-2 px-4 text-sm rounded-md border",
                        p { class: "font-medium", "عنوان الشحن" }
                        p { class: "text-muted-foreground", "١٠٠ شارع السوق، الرياض" }
                    }
                    div { class: "py-2 px-4 text-sm rounded-md border",
                        p { class: "font-medium", "العناصر" }
                        p { class: "text-muted-foreground", "٢x سماعات استوديو" }
                    }
                }
            }
        }
    }
}
