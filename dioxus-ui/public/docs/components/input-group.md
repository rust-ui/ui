+++
title = "Input Group"
description = "A component that combines inputs with addons like icons, text, or buttons."
tags = ["input"]
is_new = false
image = "/images/thumbnails/input.webp"
image_dark = "/images/thumbnails/input-dark.webp"
+++

<StaticInputGroup />

## Installation

<StaticInstallInputGroup />

## Components

The InputGroup component is composed of several subcomponents:

- **InputGroup**: Main wrapper component for input and addons
- **InputGroupInput**: The input field element
- **InputGroupAddon**: Addon container for icons, text, or buttons
- **InputGroupText**: Text helper for prefixes, suffixes, and status labels
- **InputGroupButton**: Compact button helper for addon actions

## Usage

```rust
use registry::ui::input_group::{InputGroup, InputGroupAddon, InputGroupInput};
```

```rust
rsx! {
    InputGroup {
        InputGroupInput { placeholder: "Search..." }
        InputGroupAddon {
            Search { class: "size-4" }
        }
    }
}
```

## Examples

### Text Addons

<StaticInputGroupText />

### Block Layout

<StaticInputGroupBlock />

### Custom Control

<StaticInputGroupCustom />

### Dropdown

<StaticInputGroupDropdown />

### In Card

<StaticInputGroupInCard />

### Kbd

<StaticInputGroupKbd />

### Spinner

<StaticInputGroupSpinner />

### Tooltip

<StaticInputGroupTooltip />

### Kbd Input Group

<StaticKbdInputGroup />

### Empty State

<StaticEmptyInputGroup />

### RTL

<StaticInputGroupRtl />

## See Also

- [Input](/components/input)
- [Button](/components/button)
- [Label](/components/label)
