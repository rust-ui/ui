use icons::{ChevronDown, Ellipsis};
use leptos::prelude::*;

use crate::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuTrigger,
};
use crate::ui::input_group::{
    InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupButton, InputGroupButtonSize, InputGroupInput,
};

#[component]
pub fn DemoInputGroupDropdown() -> impl IntoView {
    view! {
        <div class="grid gap-4 w-full max-w-sm">
            <InputGroup>
                <InputGroupInput placeholder="Enter file name" />
                <InputGroupAddon align=InputGroupAddonAlign::InlineEnd>
                    <DropdownMenu>
                        <DropdownMenuTrigger as_child=true>
                            <InputGroupButton size=InputGroupButtonSize::IconXs attr:aria-label="More options">
                                <Ellipsis />
                            </InputGroupButton>
                        </DropdownMenuTrigger>
                        <DropdownMenuContent>
                            <DropdownMenuGroup>
                                <DropdownMenuItem>"Settings"</DropdownMenuItem>
                                <DropdownMenuItem>"Copy path"</DropdownMenuItem>
                                <DropdownMenuItem>"Open location"</DropdownMenuItem>
                            </DropdownMenuGroup>
                        </DropdownMenuContent>
                    </DropdownMenu>
                </InputGroupAddon>
            </InputGroup>

            <InputGroup class="[--radius:1rem]">
                <InputGroupInput placeholder="Enter search query" />
                <InputGroupAddon align=InputGroupAddonAlign::InlineEnd>
                    <DropdownMenu>
                        <DropdownMenuTrigger as_child=true>
                            <InputGroupButton size=InputGroupButtonSize::Xs class="gap-1 text-xs pr-2!">
                                "Search in..."
                                <ChevronDown class="size-3" />
                            </InputGroupButton>
                        </DropdownMenuTrigger>
                        <DropdownMenuContent>
                            <DropdownMenuGroup>
                                <DropdownMenuItem>"Documentation"</DropdownMenuItem>
                                <DropdownMenuItem>"Blog Posts"</DropdownMenuItem>
                                <DropdownMenuItem>"Changelog"</DropdownMenuItem>
                            </DropdownMenuGroup>
                        </DropdownMenuContent>
                    </DropdownMenu>
                </InputGroupAddon>
            </InputGroup>
        </div>
    }
}
