+++
title = "Button Group"
description = "A component that groups multiple buttons together with shared borders and styling."
tags = ["button"]
is_new = true
image = "/images/thumbnails/button.webp"
image_dark = "/images/thumbnails/button-dark.webp"
+++




<StaticButtonGroup />





## Installation

<StaticInstallButtonGroup />



## Components

- **ButtonGroup**: Main wrapper with `orientation` prop (`Horizontal` / `Vertical`)
- **ButtonGroupText**: Muted read-only text block that looks like a button
- **ButtonGroupSeparator**: Visual separator between adjacent buttons



## Usage

You can use the `ButtonGroup` component in combination with the [Button](/docs/components/button) component.

```rust
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::button_group::ButtonGroup;
```

```rust
<ButtonGroup>
    <Button variant=ButtonVariant::Outline>"First"</Button>
    <Button variant=ButtonVariant::Outline>"Second"</Button>
    <Button variant=ButtonVariant::Outline>"Third"</Button>
</ButtonGroup>
```




## Examples

### With Separator

Button Group with visual separators between adjacent [Button](/docs/components/button) components for improved visual clarity. This example demonstrates how to use ButtonGroupSeparator in Leptos to create segmented button controls with clear boundaries and enhanced usability in Rust UI components.

<StaticButtonGroupSeparator />


### With Icons

Use `ButtonSize::Icon` with a vertical `orientation` for compact toolbar controls.

<StaticButtonGroupIcon />

### Sizes

All three sizes — `Sm`, `Default`, and `Lg` — work seamlessly with `ButtonGroup`.

<StaticButtonGroupSizes />

### With Select

Combine a [Select](/docs/components/select) with a button for protocol pickers, region selectors, or any prefix/suffix option.

<StaticButtonGroupSelect />

### With Input

Combine an [Input](/docs/components/input) with a button for search bars, URL fields, or subscription forms.

<StaticButtonGroupInput />


### RTL

Grouped buttons with Arabic labels. Border-radius logic reverses so the first and last items are correctly rounded on the RTL side.

<StaticButtonGroupRtl />

## See Also

- [Button](/docs/components/button)
- [Button Action](/docs/components/button-action)
