use dioxus::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::menubar::{
    Menubar, MenubarContent, MenubarGroup, MenubarItem, MenubarMenu, MenubarSeparator, MenubarShortcut, MenubarSub,
    MenubarSubContent, MenubarSubItem, MenubarSubTrigger, MenubarTrigger,
};

#[component]
pub fn DemoMenubarRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "w-full max-w-xl",
            Menubar {
                // ── ملف ──
                MenubarMenu {
                    MenubarTrigger { "ملف" }
                    MenubarContent {
                        MenubarGroup {
                            MenubarItem { "تبويب جديد" MenubarShortcut { "⌘T" } }
                            MenubarItem { "نافذة جديدة" MenubarShortcut { "⌘N" } }
                        }
                        MenubarSeparator {}
                        MenubarGroup {
                            MenubarSub {
                                MenubarSubTrigger { "مشاركة" }
                                MenubarSubContent {
                                    MenubarSubItem { "رابط البريد الإلكتروني" }
                                    MenubarSubItem { "الرسائل" }
                                }
                            }
                        }
                        MenubarSeparator {}
                        MenubarGroup {
                            MenubarItem { "طباعة..." MenubarShortcut { "⌘P" } }
                        }
                    }
                }

                // ── تحرير ──
                MenubarMenu {
                    MenubarTrigger { "تحرير" }
                    MenubarContent {
                        MenubarGroup {
                            MenubarItem { "تراجع" MenubarShortcut { "⌘Z" } }
                            MenubarItem { "إعادة" MenubarShortcut { "⇧⌘Z" } }
                        }
                        MenubarSeparator {}
                        MenubarGroup {
                            MenubarItem { "قص" }
                            MenubarItem { "نسخ" }
                            MenubarItem { "لصق" }
                        }
                    }
                }

                // ── عرض ──
                MenubarMenu {
                    MenubarTrigger { "عرض" }
                    MenubarContent {
                        MenubarGroup {
                            MenubarItem { "إعادة تحميل" MenubarShortcut { "⌘R" } }
                        }
                    }
                }
            }
        }
    }
}
