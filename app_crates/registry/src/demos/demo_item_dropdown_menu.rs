use dioxus::prelude::*;

use crate::ui::avatar::{Avatar, AvatarFallback};
use crate::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAlign, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use crate::ui::item::{Item, ItemContent, ItemDescription, ItemMedia, ItemSize, ItemTitle};

#[component]
pub fn DemoItemDropdownMenu() -> Element {
    rsx! {
        DropdownMenu { align: DropdownMenuAlign::End,
            DropdownMenuTrigger { class: "w-fit", "Dropdown" }

            DropdownMenuContent { class: "w-72 [--radius:0.65rem]",
                for person in PEOPLE {
                    DropdownMenuItem { class: "p-0",
                        Item { size: ItemSize::Sm, class: "p-2 w-full",
                            ItemMedia {
                                Avatar { class: "size-8",
                                    AvatarFallback { "{person.initials}" }
                                }
                            }
                            ItemContent { class: "gap-0.5",
                                ItemTitle { "{person.username}" }
                                ItemDescription { "{person.email}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

/* ========================================================== */
/*                     CONSTANTS                              */
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
