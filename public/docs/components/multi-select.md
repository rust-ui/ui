+++
title = "MultiSelect"
description = "Rust/UI component that displays a dropdown menu that allows the user to select several options."
tags = ["select"]
is_new = false
image = "/images/thumbnails/select.webp"
image_dark = "/images/thumbnails/select-dark.webp"
+++


<StaticMultiSelect />




## Installation

<StaticInstallMultiSelect />



## Components

The MultiSelect component is composed of several subcomponents:

- **MultiSelect**: Main wrapper component managing multi-selection state
- **MultiSelectTrigger**: Button that opens the dropdown menu
- **MultiSelectValue**: Displays the currently selected values
- **MultiSelectContent**: Dropdown container for options
- **MultiSelectGroup**: Groups related options together
- **MultiSelectLabel**: Label text for option groups
- **MultiSelectItem**: Individual option container
- **MultiSelectOption**: Selectable option with value



## Usage

```rust
use crate::components::ui::multi_select::{
    MultiSelect,
    MultiSelectContent,
    MultiSelectGroup,
    MultiSelectItem,
    MultiSelectLabel,
    MultiSelectOption,
    MultiSelectTrigger,
    MultiSelectValue,
};
```

```rust
<MultiSelect>
    <MultiSelectTrigger>
        <MultiSelectValue placeholder="Select options" />
    </MultiSelectTrigger>
    <MultiSelectContent>
        <MultiSelectGroup>
            <MultiSelectLabel>"Options"</MultiSelectLabel>
            <MultiSelectItem>
                <MultiSelectOption value="option1">"Option 1"</MultiSelectOption>
            </MultiSelectItem>
            <MultiSelectItem>
                <MultiSelectOption value="option2">"Option 2"</MultiSelectOption>
            </MultiSelectItem>
        </MultiSelectGroup>
    </MultiSelectContent>
</MultiSelect>
```



## Examples

### Scrollable

A multi-select component with grouped timezone options that scrolls when content exceeds the maximum height.

<StaticMultiSelectScrollable />

### Align Start & End

MultiSelect supports horizontal alignment via `MultiSelectAlign`. Use `Start` to anchor the dropdown to the left edge or `End` to anchor to the right edge of the trigger element.

<StaticMultiSelectAlign />


## See Also

- [Select](/docs/components/select)
- [Combobox](/docs/components/combobox)
- [Form](/docs/components/form)
- [Chips](/docs/components/chips)
