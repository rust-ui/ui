use leptos::prelude::*;

use crate::ui::context_menu::{
    ContextMenu, ContextMenuAction, ContextMenuContent, ContextMenuGroup, ContextMenuItem, ContextMenuLabel,
    ContextMenuSub, ContextMenuSubContent, ContextMenuSubItem, ContextMenuSubTrigger, ContextMenuTrigger,
};
use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::separator::Separator;

#[component]
pub fn DemoContextMenuRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <ContextMenu>
                <ContextMenuTrigger class="flex justify-center items-center text-sm rounded-md border border-dashed h-[150px] w-[300px]">
                    "انقر بالزر الأيمن هنا"
                </ContextMenuTrigger>

                <ContextMenuContent>
                    <ContextMenuLabel>"الإجراءات"</ContextMenuLabel>

                    <ContextMenuGroup>
                        <ContextMenuItem>
                            <ContextMenuAction>"رجوع"</ContextMenuAction>
                        </ContextMenuItem>
                        <ContextMenuItem>
                            <ContextMenuAction>"للأمام"</ContextMenuAction>
                        </ContextMenuItem>
                        <ContextMenuItem>
                            <ContextMenuAction>"إعادة تحميل"</ContextMenuAction>
                        </ContextMenuItem>

                        <ContextMenuSub>
                            <ContextMenuSubTrigger>"أدوات إضافية"</ContextMenuSubTrigger>
                            <ContextMenuSubContent>
                                <ContextMenuSubItem>"حفظ الصفحة بصيغة..."</ContextMenuSubItem>
                                <ContextMenuSubItem>"إنشاء اختصار..."</ContextMenuSubItem>
                                <ContextMenuSubItem>"تسمية النافذة..."</ContextMenuSubItem>
                                <Separator class="my-1" />
                                <ContextMenuSubItem>"أدوات المطور"</ContextMenuSubItem>
                            </ContextMenuSubContent>
                        </ContextMenuSub>
                    </ContextMenuGroup>

                    <Separator class="my-1" />

                    <ContextMenuGroup>
                        <ContextMenuItem>
                            <ContextMenuAction>
                                "إظهار شريط الإشارات المرجعية"
                            </ContextMenuAction>
                        </ContextMenuItem>
                        <ContextMenuItem>
                            <ContextMenuAction>"إظهار عناوين URL الكاملة"</ContextMenuAction>
                        </ContextMenuItem>
                    </ContextMenuGroup>
                </ContextMenuContent>
            </ContextMenu>
        </DirectionProvider>
    }
}
