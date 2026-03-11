use leptos::prelude::*;

use crate::ui::avatar::{Avatar, AvatarFallback};
use crate::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAlign, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use crate::ui::item::{Item, ItemContent, ItemDescription, ItemMedia, ItemSize, ItemTitle};

#[component]
pub fn DemoItemDropdownMenu() -> impl IntoView {
    view! {
        <DropdownMenu align=DropdownMenuAlign::End>
            <DropdownMenuTrigger class="w-fit">"Dropdown"</DropdownMenuTrigger>

            <DropdownMenuContent class="w-72 [--radius:0.65rem]">
                {PEOPLE
                    .iter()
                    .map(|person| {
                        view! {
                            <DropdownMenuItem class="p-0">
                                <Item size=ItemSize::Sm class="p-2 w-full">
                                    <ItemMedia>
                                        <Avatar class="size-8">
                                            <AvatarFallback>{person.initials}</AvatarFallback>
                                        </Avatar>
                                    </ItemMedia>
                                    <ItemContent class="gap-0.5">
                                        <ItemTitle>{person.username}</ItemTitle>
                                        <ItemDescription>{person.email}</ItemDescription>
                                    </ItemContent>
                                </Item>
                            </DropdownMenuItem>
                        }
                    })
                    .collect::<Vec<_>>()}
            </DropdownMenuContent>
        </DropdownMenu>
    }
}

/* ========================================================== */
/*                     ✨ CONSTANTS ✨                        */
/* ========================================================== */

#[derive(Clone)]
struct Person {
    username: &'static str,
    initials: &'static str,
    email: &'static str,
}

const PEOPLE: &[Person] = &[
    Person { username: "Ryan Smith", initials: "RS", email: "ryan.smith@example.com" },
    Person { username: "Morgan Williams", initials: "MW", email: "morgan.williams@example.com" },
    Person { username: "Max Murphy", initials: "MM", email: "max.murphy@example.com" },
];
