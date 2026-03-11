use leptos::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::menubar::{
    Menubar, MenubarContent, MenubarGroup, MenubarItem, MenubarMenu, MenubarSeparator, MenubarShortcut, MenubarSub,
    MenubarSubContent, MenubarSubItem, MenubarSubTrigger, MenubarTrigger,
};

#[component]
pub fn DemoMenubarRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-xl">
            <Menubar>
                // ── ملف ──
                <MenubarMenu>
                    <MenubarTrigger>"ملف"</MenubarTrigger>
                    <MenubarContent>
                        <MenubarGroup>
                            <MenubarItem>"تبويب جديد" <MenubarShortcut>"⌘T"</MenubarShortcut></MenubarItem>
                            <MenubarItem>"نافذة جديدة" <MenubarShortcut>"⌘N"</MenubarShortcut></MenubarItem>
                        </MenubarGroup>
                        <MenubarSeparator />
                        <MenubarGroup>
                            <MenubarSub>
                                <MenubarSubTrigger>"مشاركة"</MenubarSubTrigger>
                                <MenubarSubContent>
                                    <MenubarSubItem>"رابط البريد الإلكتروني"</MenubarSubItem>
                                    <MenubarSubItem>"الرسائل"</MenubarSubItem>
                                </MenubarSubContent>
                            </MenubarSub>
                        </MenubarGroup>
                        <MenubarSeparator />
                        <MenubarGroup>
                            <MenubarItem>"طباعة..." <MenubarShortcut>"⌘P"</MenubarShortcut></MenubarItem>
                        </MenubarGroup>
                    </MenubarContent>
                </MenubarMenu>

                // ── تحرير ──
                <MenubarMenu>
                    <MenubarTrigger>"تحرير"</MenubarTrigger>
                    <MenubarContent>
                        <MenubarGroup>
                            <MenubarItem>"تراجع" <MenubarShortcut>"⌘Z"</MenubarShortcut></MenubarItem>
                            <MenubarItem>"إعادة" <MenubarShortcut>"⇧⌘Z"</MenubarShortcut></MenubarItem>
                        </MenubarGroup>
                        <MenubarSeparator />
                        <MenubarGroup>
                            <MenubarItem>"قص"</MenubarItem>
                            <MenubarItem>"نسخ"</MenubarItem>
                            <MenubarItem>"لصق"</MenubarItem>
                        </MenubarGroup>
                    </MenubarContent>
                </MenubarMenu>

                // ── عرض ──
                <MenubarMenu>
                    <MenubarTrigger>"عرض"</MenubarTrigger>
                    <MenubarContent>
                        <MenubarGroup>
                            <MenubarItem>"إعادة تحميل" <MenubarShortcut>"⌘R"</MenubarShortcut></MenubarItem>
                        </MenubarGroup>
                    </MenubarContent>
                </MenubarMenu>
            </Menubar>
        </DirectionProvider>
    }
}
