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
use crate::ui::command::{
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
rsx! {
    Command {
        CommandInput { placeholder: "Type a command or search..." }
        CommandList {
            CommandEmpty { "No results found." }
            CommandGroup {
                CommandGroupLabel { "Suggestions" }
                CommandItemLink { href: "/calendar", "Calendar" }
                CommandItemLink { href: "/search", "Search" }
                CommandItemLink { href: "/settings", "Settings" }
            }
        }
    }
}
```

## Examples

### Inline Command

Inline command menu for filtering and selecting actions inside a page or panel.

<StaticCommand />

### Command Dialog

Fast command palette dialog with grouped actions, keyboard shortcuts, and search input.

<StaticCommandDialog />

## See Also

- [Combobox](/components/combobox)
- [Kbd](/components/kbd)
