use icons::{Pencil, Share2, Trash2};
use leptos::prelude::*;

use crate::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAction, DropdownMenuActionVariant, DropdownMenuContent, DropdownMenuGroup,
    DropdownMenuItem, DropdownMenuTrigger,
};
use crate::ui::separator::Separator;

#[component]
pub fn DemoDropdownMenuDestructive() -> impl IntoView {
    view! {
        <DropdownMenu>
            <DropdownMenuTrigger>"Actions"</DropdownMenuTrigger>

            <DropdownMenuContent>
                <DropdownMenuGroup>
                    <DropdownMenuItem>
                        <DropdownMenuAction>
                            <Pencil />
                            "Edit"
                        </DropdownMenuAction>
                    </DropdownMenuItem>
                    <DropdownMenuItem>
                        <DropdownMenuAction>
                            <Share2 />
                            "Share"
                        </DropdownMenuAction>
                    </DropdownMenuItem>
                </DropdownMenuGroup>

                <Separator class="my-1" />

                <DropdownMenuGroup>
                    <DropdownMenuItem>
                        <DropdownMenuAction variant=DropdownMenuActionVariant::Destructive>
                            <Trash2 />
                            "Delete"
                        </DropdownMenuAction>
                    </DropdownMenuItem>
                </DropdownMenuGroup>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
