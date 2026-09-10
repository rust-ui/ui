use dioxus::prelude::*;
use icons::{ArrowDownWideNarrow, ArrowUpNarrowWide, ChevronDown, CircleX};

use crate::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAction, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuRadioGroup,
    DropdownMenuRadioItem, DropdownMenuSeparator, DropdownMenuTrigger,
};

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum SortDirection {
    #[default]
    None,
    Asc,
    Desc,
}

impl SortDirection {
    fn label(self) -> &'static str {
        match self {
            SortDirection::None => "No sort",
            SortDirection::Asc => "Ascending",
            SortDirection::Desc => "Descending",
        }
    }
}

#[component]
pub fn DemoDropdownMenuRadio() -> Element {
    let sort_signal = use_signal(SortDirection::default);

    rsx! {
        DropdownMenu {
            DropdownMenuTrigger { class: "flex gap-2 justify-between items-center",
                span { "Sort: " }
                span { class: "text-muted-foreground", "{sort_signal().label()}" }
                ChevronDown { class: "size-4 text-muted-foreground" }
            }

            DropdownMenuContent {
                DropdownMenuRadioGroup { value: sort_signal,
                    DropdownMenuRadioItem { value: SortDirection::Asc,
                        ArrowUpNarrowWide { class: "text-muted-foreground" }
                        "Sort asc"
                    }
                    DropdownMenuRadioItem { value: SortDirection::Desc,
                        ArrowDownWideNarrow { class: "text-muted-foreground" }
                        "Sort desc"
                    }
                }

                if sort_signal() != SortDirection::None {
                    DropdownMenuItem {
                        onclick: move |_| sort_signal.clone().set(SortDirection::None),
                        DropdownMenuAction {
                            CircleX { class: "text-muted-foreground" }
                            "Remove sort"
                        }
                    }
                }

                DropdownMenuSeparator {}

                DropdownMenuGroup {
                    DropdownMenuItem {
                        DropdownMenuAction { "Other action" }
                    }
                }
            }
        }
    }
}
