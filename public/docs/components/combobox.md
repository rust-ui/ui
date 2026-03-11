+++
title = "Combobox"
description = "Autocomplete input and command palette with a list of suggestions."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++


<StaticCombobox />



## Installation

The Combobox is built using a composition of the [Popover](/docs/components/popover) and the [Command](/docs/components/command) components.

See installation instructions for the [Popover](/docs/components/popover#installation) and the [Command](/docs/components/command#installation) components.



## Usage

```rust
use strum::{Display, EnumIter, IntoEnumIterator};

use crate::components::ui::command::{
    Command,
    CommandEmpty,
    CommandGroup,
    CommandInput,
    CommandItem,
    CommandList,
};
use crate::components::ui::popover::{
    Popover,
    PopoverAlign,
    PopoverContent,
    PopoverTrigger,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter)]
enum Language {
    Rust,
    JavaScript,
    Ruby,
    Python,
}
```

```rust
let value_signal = RwSignal::new(None::<Language>);

view! {
    <Popover align=PopoverAlign::Start>
        <PopoverTrigger class="justify-between w-[200px]">
            <span class="truncate">
                {move || value_signal.get().map(|l| l.to_string()).unwrap_or_else(|| "Select language...".into())}
            </span>
            <ChevronsUpDown class="ml-auto opacity-50 size-4" />
        </PopoverTrigger>

        <PopoverContent class="p-0 w-[200px]">
            <Command>
                <div class="flex gap-2 items-center px-2 border-b">
                    <Search class="size-4 text-muted-foreground shrink-0" />
                    <CommandInput attr:placeholder="Search language..." />
                </div>
                <CommandList>
                    <CommandEmpty>"No language found."</CommandEmpty>
                    <CommandGroup>
                        {Language::iter()
                            .map(|language| {
                                let label = language.to_string();
                                let is_selected = Signal::derive(move || value_signal.get() == Some(language));
                                view! {
                                    <CommandItem
                                        value=label.clone()
                                        selected=is_selected
                                        on_select=Callback::new(move |_| {
                                            value_signal.set(Some(language));
                                        })
                                    >
                                        {label}
                                    </CommandItem>
                                }
                            })
                            .collect_view()}
                    </CommandGroup>
                </CommandList>
            </Command>
        </PopoverContent>
    </Popover>
}
```


## See Also

- [Select](/docs/components/select)
- [Command](/docs/components/command)
- [Multi Select](/docs/components/multi-select)
