use dioxus::prelude::*;
use icons::{ArrowUpRight, FolderCode};

use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::empty::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle};

#[component]
pub fn DemoEmptyRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "w-full max-w-md",
            Empty {
                EmptyHeader {
                    EmptyMedia { variant: EmptyMediaVariant::Icon,
                        FolderCode {}
                    }
                    EmptyTitle { "لا توجد مشاريع بعد" }
                    EmptyDescription {
                        "لم تقم بإنشاء أي مشاريع بعد. ابدأ بإنشاء مشروعك الأول."
                    }
                }
                EmptyContent {
                    div { class: "flex gap-2",
                        Button { "إنشاء مشروع" }
                        Button { variant: ButtonVariant::Outline, "استيراد مشروع" }
                    }
                }
                Button { variant: ButtonVariant::Link, size: ButtonSize::Sm, class: "text-muted-foreground",
                    a { href: "#", class: "flex gap-1 items-center",
                        span { "معرفة المزيد" }
                        ArrowUpRight { class: "rtl:rotate-180" }
                    }
                }
            }
        }
    }
}
