use dioxus::prelude::*;

use crate::ui::context_menu::{
    ContextMenu, ContextMenuAction, ContextMenuContent, ContextMenuGroup, ContextMenuItem, ContextMenuLabel,
    ContextMenuSub, ContextMenuSubContent, ContextMenuSubItem, ContextMenuSubTrigger, ContextMenuTrigger,
};
use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::separator::Separator;

#[component]
pub fn DemoContextMenuRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "max-w-fit",
            ContextMenu {
                ContextMenuTrigger {
                    class: "flex justify-center items-center text-sm rounded-md border border-dashed h-[150px] w-[300px]",
                    "انقر بالزر الأيمن هنا"
                }

                ContextMenuContent {
                    ContextMenuLabel { "الإجراءات" }

                    ContextMenuGroup {
                        ContextMenuItem {
                            ContextMenuAction { "رجوع" }
                        }
                        ContextMenuItem {
                            ContextMenuAction { "للأمام" }
                        }
                        ContextMenuItem {
                            ContextMenuAction { "إعادة تحميل" }
                        }

                        ContextMenuSub {
                            ContextMenuSubTrigger { "أدوات إضافية" }
                            ContextMenuSubContent {
                                ContextMenuSubItem { "حفظ الصفحة بصيغة..." }
                                ContextMenuSubItem { "إنشاء اختصار..." }
                                ContextMenuSubItem { "تسمية النافذة..." }
                                Separator { class: "my-1" }
                                ContextMenuSubItem { "أدوات المطور" }
                            }
                        }
                    }

                    Separator { class: "my-1" }

                    ContextMenuGroup {
                        ContextMenuItem {
                            ContextMenuAction { "إظهار شريط الإشارات المرجعية" }
                        }
                        ContextMenuItem {
                            ContextMenuAction { "إظهار عناوين URL الكاملة" }
                        }
                    }
                }
            }
        }
    }
}
