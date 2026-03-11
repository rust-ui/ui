+++
title = "Select"
description = "Rust/UI component that displays a dropdown menu that allows the user to select an option."
tags = ["select"]
is_new = false
image = "/images/thumbnails/select.webp"
image_dark = "/images/thumbnails/select-dark.webp"
+++


<StaticSelect />




## Installation

<StaticInstallSelect />



## Components

The Select component is composed of several subcomponents:

- **Select**: Main wrapper component managing selection state
- **SelectTrigger**: Button that opens the dropdown menu
- **SelectValue**: Displays the currently selected value
- **SelectContent**: Dropdown container for options
- **SelectGroup**: Groups related options together
- **SelectLabel**: Label text for option groups
- **SelectItem**: Individual option container
- **SelectOption**: Selectable option with value



## Usage

```rust
use crate::components::ui::select::{
    Select,
    SelectContent,
    SelectGroup,
    SelectItem,
    SelectLabel,
    SelectOption,
    SelectTrigger,
    SelectValue,
};
```

```rust
<Select>
    <SelectTrigger>
        <SelectValue placeholder="Select an option" />
    </SelectTrigger>
    <SelectContent>
        <SelectGroup>
            <SelectLabel>"Options"</SelectLabel>
            <SelectItem>
                <SelectOption value="option1">"Option 1"</SelectOption>
            </SelectItem>
            <SelectItem>
                <SelectOption value="option2">"Option 2"</SelectOption>
            </SelectItem>
        </SelectGroup>
    </SelectContent>
</Select>
```



## Examples

### Scrollable

A select component with grouped timezone options that scrolls when content exceeds the maximum height.

<StaticSelectScrollable />


### RTL

Select dropdown with Arabic options. The trigger value and dropdown content align to the right in RTL mode.

<StaticSelectRtl />

## See Also

- [Form](/docs/components/form)
- [Multi Select](/docs/components/multi-select)
- [Combobox](/docs/components/combobox)
