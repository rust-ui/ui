+++
title = "Radio Button Group"
description = "Rust/UI component that displays a group of radio buttons."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++



<StaticRadioButtonGroup />








## Installation

<StaticInstallRadioButtonGroup />




## Components

The RadioButtonGroup component is composed of several subcomponents:

- **RadioButtonGroup**: Main wrapper component managing selection state
- **RadioButton**: Individual radio button option
- **RadioButtonText**: Text label for the radio button



## Usage

```rust
use crate::components::ui::radio_button_group::{RadioButtonGroup, RadioButton, RadioButtonText};
```

```rust
<RadioButtonGroup>
    <RadioButton checked=true>
        <RadioButtonText>"Option 1"</RadioButtonText>
    </RadioButton>
    <RadioButton>
        <RadioButtonText>"Option 2"</RadioButtonText>
    </RadioButton>
    <RadioButton>
        <RadioButtonText>"Option 3"</RadioButtonText>
    </RadioButton>
</RadioButtonGroup>
```


### RTL

Radio button group with Arabic options. The selected-state indicator and label layout reverse to match RTL reading flow.

<StaticRadioButtonGroupRtl />

## See Also

- [Radio Button](/docs/components/radio-button)
- [Form](/docs/components/form)
