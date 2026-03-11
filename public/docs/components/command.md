+++
title = "Command"
description = "Fast, composable, unstyled command menu for Leptos."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++


<StaticCommand />



## Installation

<StaticInstallCommand />


## Components

The Command component is composed of several subcomponents:

- **Command**: Main command palette wrapper component
- **CommandDialog**: Modal dialog variant of the command palette
- **CommandDialogProvider**: Context provider for dialog functionality
- **CommandDialogTrigger**: Button or element that opens the command dialog
- **CommandInput**: Search input field with filtering
- **CommandList**: Scrollable list container for items
- **CommandEmpty**: Message shown when no results found
- **CommandGroup**: Groups related command items together
- **CommandGroupLabel**: Label text for command groups
- **CommandItem**: Individual selectable command item
- **CommandItemLink**: Command item as a navigation link
- **CommandHeader**: Header section for title and description
- **CommandTitle**: Primary heading text
- **CommandDescription**: Secondary descriptive text
- **CommandFooter**: Footer section for additional actions



## Usage

```rust
use crate::components::ui::command::{
    Command,
    CommandDialog,
    CommandDialogProvider,
    CommandDialogTrigger,
    CommandInput,
    CommandList,
    CommandEmpty,
    CommandGroup,
    CommandGroupLabel,
    CommandItem,
    CommandItemLink,
    CommandHeader,
    CommandTitle,
    CommandDescription,
    CommandFooter,
};
```

```rust
<Command>
    <CommandInput placeholder="Type a command or search..." />
    <CommandList>
        <CommandEmpty>"No results found."</CommandEmpty>
        <CommandGroup>
            <CommandGroupLabel>"Suggestions"</CommandGroupLabel>
            <CommandItemLink href="/calendar">"Calendar"</CommandItemLink>
            <CommandItemLink href="/search">"Search"</CommandItemLink>
            <CommandItemLink href="/settings">"Settings"</CommandItemLink>
        </CommandGroup>
    </CommandList>
</Command>
```




## Examples

### Command Dialog

Fast command palette [Dialog](/docs/components/dialog) for keyboard-driven navigation and search. This example demonstrates how to build accessible command menus in Leptos with fuzzy search, keyboard shortcuts, and grouped actions for improved productivity in Rust applications.

<StaticCommandDialog />


## See Also

- [Combobox](/docs/components/combobox)
- [Dropdown Menu](/docs/components/dropdown-menu)
- [Kbd](/docs/components/kbd)
