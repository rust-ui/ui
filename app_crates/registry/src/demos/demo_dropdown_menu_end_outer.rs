use leptos::prelude::*;

use crate::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAction, DropdownMenuAlign, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem,
    DropdownMenuLabel, DropdownMenuLink, DropdownMenuSub, DropdownMenuSubContent, DropdownMenuSubItem,
    DropdownMenuSubTrigger, DropdownMenuTrigger,
};
use crate::ui::separator::Separator;

#[component]
pub fn DemoDropdownMenuEndOuter() -> impl IntoView {
    view! {
        <DropdownMenu align=DropdownMenuAlign::EndOuter>
            <DropdownMenuTrigger>"Open (EndOuter)"</DropdownMenuTrigger>

            <DropdownMenuContent>
                <DropdownMenuLabel>"EndOuter Menu"</DropdownMenuLabel>

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
