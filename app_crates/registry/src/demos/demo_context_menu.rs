use leptos::prelude::*;

use crate::ui::context_menu::{
    ContextMenu, ContextMenuAction, ContextMenuContent, ContextMenuGroup, ContextMenuItem, ContextMenuLabel,
    ContextMenuSub, ContextMenuSubContent, ContextMenuSubItem, ContextMenuSubTrigger, ContextMenuTrigger,
};
use crate::ui::separator::Separator;

#[component]
pub fn DemoContextMenu() -> impl IntoView {
    view! {
        <ContextMenu>
            <ContextMenuTrigger class="flex justify-center items-center text-sm rounded-md border border-dashed h-[150px] w-[300px]">
                "Right click here"
            </ContextMenuTrigger>

            <ContextMenuContent>
                <ContextMenuLabel>"Actions"</ContextMenuLabel>

                <ContextMenuGroup>
                    <ContextMenuItem>
                        <ContextMenuAction>"Back"</ContextMenuAction>
                    </ContextMenuItem>
                    <ContextMenuItem>
                        <ContextMenuAction>"Forward"</ContextMenuAction>
                    </ContextMenuItem>
                    <ContextMenuItem>
                        <ContextMenuAction>"Reload"</ContextMenuAction>
                    </ContextMenuItem>

                    <ContextMenuSub>
                        <ContextMenuSubTrigger>"More Tools"</ContextMenuSubTrigger>
                        <ContextMenuSubContent>
                            <ContextMenuSubItem>"Save Page As..."</ContextMenuSubItem>
                            <ContextMenuSubItem>"Create Shortcut..."</ContextMenuSubItem>
                            <ContextMenuSubItem>"Name Window..."</ContextMenuSubItem>
                            <Separator class="my-1" />
                            <ContextMenuSubItem>"Developer Tools"</ContextMenuSubItem>
                        </ContextMenuSubContent>
                    </ContextMenuSub>
                </ContextMenuGroup>

                <Separator class="my-1" />

                <ContextMenuGroup>
                    <ContextMenuItem>
                        <ContextMenuAction>"Show Bookmarks Bar"</ContextMenuAction>
                    </ContextMenuItem>
                    <ContextMenuItem>
                        <ContextMenuAction>"Show Full URLs"</ContextMenuAction>
                    </ContextMenuItem>
                </ContextMenuGroup>
            </ContextMenuContent>
        </ContextMenu>
    }
}
