use leptos::prelude::*;

use crate::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAction, DropdownMenuAlign, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem,
    DropdownMenuLabel, DropdownMenuLink, DropdownMenuSub, DropdownMenuSubContent, DropdownMenuSubItem,
    DropdownMenuSubTrigger, DropdownMenuTrigger,
};
use crate::ui::separator::Separator;

#[component]
pub fn DemoDropdownMenuUserIcon() -> impl IntoView {
    view! {
        <DropdownMenu align=DropdownMenuAlign::EndOuter>
            <DropdownMenuTrigger class="p-0 bg-transparent border-0">
                <div class="flex gap-2 items-center">
                    <span data-name="avatar" class="flex overflow-hidden relative rounded-lg size-8 shrink-0">
                        <span
                            data-name="avatar-fallback"
                            class="flex justify-center items-center rounded-lg bg-secondary size-full"
                        >
                            RS
                        </span>
                    </span>
                </div>
            </DropdownMenuTrigger>

            <DropdownMenuContent class="w-[220px]">
                <DropdownMenuLabel>"Main Menu"</DropdownMenuLabel>

                <DropdownMenuGroup>
                    <DropdownMenuItem>
                        <DropdownMenuAction>"Simple Item"</DropdownMenuAction>
                    </DropdownMenuItem>

                    <DropdownMenuSub>
                        <DropdownMenuSubTrigger>"Settings"</DropdownMenuSubTrigger>
                        <DropdownMenuSubContent>
                            <DropdownMenuSubItem>"Account Settings"</DropdownMenuSubItem>
                            <DropdownMenuSubItem>"Privacy Settings"</DropdownMenuSubItem>
                            <DropdownMenuSubItem>"Notification Settings"</DropdownMenuSubItem>
                        </DropdownMenuSubContent>
                    </DropdownMenuSub>

                    <DropdownMenuSub>
                        <DropdownMenuSubTrigger>"Tools"</DropdownMenuSubTrigger>
                        <DropdownMenuSubContent>
                            <DropdownMenuSubItem>"Export Data"</DropdownMenuSubItem>
                            <DropdownMenuSubItem>"Import Data"</DropdownMenuSubItem>
                            <Separator class="my-1" />
                            <DropdownMenuSubItem>"Developer Tools"</DropdownMenuSubItem>
                        </DropdownMenuSubContent>
                    </DropdownMenuSub>
                </DropdownMenuGroup>

                <Separator class="my-1" />

                <DropdownMenuGroup>
                    <DropdownMenuItem>
                        <DropdownMenuLink attr:href="/">"Home"</DropdownMenuLink>
                    </DropdownMenuItem>
                    <DropdownMenuItem>
                        <DropdownMenuAction>"Sign Out"</DropdownMenuAction>
                    </DropdownMenuItem>
                </DropdownMenuGroup>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
