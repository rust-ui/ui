use dioxus::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAction, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuLabel,
    DropdownMenuLink, DropdownMenuSub, DropdownMenuSubContent, DropdownMenuSubItem, DropdownMenuSubTrigger,
    DropdownMenuTrigger,
};
use crate::ui::separator::Separator;

#[component]
pub fn DemoDropdownMenuRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "max-w-fit",
            DropdownMenu {
                DropdownMenuTrigger { "فتح القائمة" }

                DropdownMenuContent {
                    DropdownMenuLabel { "القائمة الرئيسية" }

                    DropdownMenuGroup {
                        DropdownMenuItem {
                            DropdownMenuAction { "عنصر بسيط" }
                        }

                        DropdownMenuSub {
                            DropdownMenuSubTrigger { "الإعدادات" }
                            DropdownMenuSubContent {
                                DropdownMenuSubItem { "إعدادات الحساب" }
                                DropdownMenuSubItem { "إعدادات الخصوصية" }
                                DropdownMenuSubItem { "إعدادات الإشعارات" }
                            }
                        }

                        DropdownMenuSub {
                            DropdownMenuSubTrigger { "الأدوات" }
                            DropdownMenuSubContent {
                                DropdownMenuSubItem { "تصدير البيانات" }
                                DropdownMenuSubItem { "استيراد البيانات" }
                                Separator { class: "my-1" }
                                DropdownMenuSubItem { "أدوات المطور" }
                            }
                        }
                    }

                    Separator { class: "my-1" }

                    DropdownMenuGroup {
                        DropdownMenuItem {
                            DropdownMenuLink { href: "/", "الرئيسية" }
                        }
                        DropdownMenuItem {
                            DropdownMenuAction { "تسجيل الخروج" }
                        }
                    }
                }
            }
        }
    }
}
