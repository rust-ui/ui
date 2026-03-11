use leptos::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAction, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuLabel,
    DropdownMenuLink, DropdownMenuSub, DropdownMenuSubContent, DropdownMenuSubItem, DropdownMenuSubTrigger,
    DropdownMenuTrigger,
};
use crate::ui::separator::Separator;

#[component]
pub fn DemoDropdownMenuRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <DropdownMenu>
                <DropdownMenuTrigger>"فتح القائمة"</DropdownMenuTrigger>

                <DropdownMenuContent>
                    <DropdownMenuLabel>"القائمة الرئيسية"</DropdownMenuLabel>

                    <DropdownMenuGroup>
                        <DropdownMenuItem>
                            <DropdownMenuAction>"عنصر بسيط"</DropdownMenuAction>
                        </DropdownMenuItem>

                        <DropdownMenuSub>
                            <DropdownMenuSubTrigger>"الإعدادات"</DropdownMenuSubTrigger>
                            <DropdownMenuSubContent>
                                <DropdownMenuSubItem>"إعدادات الحساب"</DropdownMenuSubItem>
                                <DropdownMenuSubItem>"إعدادات الخصوصية"</DropdownMenuSubItem>
                                <DropdownMenuSubItem>"إعدادات الإشعارات"</DropdownMenuSubItem>
                            </DropdownMenuSubContent>
                        </DropdownMenuSub>

                        <DropdownMenuSub>
                            <DropdownMenuSubTrigger>"الأدوات"</DropdownMenuSubTrigger>
                            <DropdownMenuSubContent>
                                <DropdownMenuSubItem>"تصدير البيانات"</DropdownMenuSubItem>
                                <DropdownMenuSubItem>"استيراد البيانات"</DropdownMenuSubItem>
                                <Separator class="my-1" />
                                <DropdownMenuSubItem>"أدوات المطور"</DropdownMenuSubItem>
                            </DropdownMenuSubContent>
                        </DropdownMenuSub>
                    </DropdownMenuGroup>

                    <Separator class="my-1" />

                    <DropdownMenuGroup>
                        <DropdownMenuItem>
                            <DropdownMenuLink attr:href="/">"الرئيسية"</DropdownMenuLink>
                        </DropdownMenuItem>
                        <DropdownMenuItem>
                            <DropdownMenuAction>"تسجيل الخروج"</DropdownMenuAction>
                        </DropdownMenuItem>
                    </DropdownMenuGroup>
                </DropdownMenuContent>
            </DropdownMenu>
        </DirectionProvider>
    }
}
