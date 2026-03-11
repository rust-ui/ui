use icons::{ArrowDownWideNarrow, ArrowUpNarrowWide, ChevronDown, CircleX};
use leptos::prelude::*;

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
pub fn DemoDropdownMenuRadio() -> impl IntoView {
    let sort_signal = RwSignal::new(SortDirection::default());

    view! {
        <DropdownMenu>
            <DropdownMenuTrigger class="flex gap-2 justify-between items-center">
                <span>"Sort: "</span>
                <span class="text-muted-foreground">{move || sort_signal.get().label()}</span>
                <ChevronDown class="size-4 text-muted-foreground" />
            </DropdownMenuTrigger>

            <DropdownMenuContent>
                <DropdownMenuRadioGroup value=sort_signal>
                    <DropdownMenuRadioItem value=SortDirection::Asc>
                        <ArrowUpNarrowWide class="text-muted-foreground" />
                        "Sort asc"
                    </DropdownMenuRadioItem>
                    <DropdownMenuRadioItem value=SortDirection::Desc>
                        <ArrowDownWideNarrow class="text-muted-foreground" />
                        "Sort desc"
                    </DropdownMenuRadioItem>
                </DropdownMenuRadioGroup>

                {move || {
                    (sort_signal.get() != SortDirection::None)
                        .then(|| {
                            view! {
                                <DropdownMenuItem on:click=move |_| sort_signal.set(SortDirection::None)>
                                    <DropdownMenuAction>
                                        <CircleX class="text-muted-foreground" />
                                        "Remove sort"
                                    </DropdownMenuAction>
                                </DropdownMenuItem>
                            }
                        })
                }}

                <DropdownMenuSeparator />

                <DropdownMenuGroup>
                    <DropdownMenuItem>
                        <DropdownMenuAction>"Other action"</DropdownMenuAction>
                    </DropdownMenuItem>
                </DropdownMenuGroup>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
