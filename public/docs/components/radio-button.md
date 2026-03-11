+++
title = "Radio Button"
description = "Rust/UI component that displays a set of checkable buttons where only one can be selected at a time."
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++


<StaticRadioButton />




## Installation

<StaticInstallRadioButton />




## Usage

You can use the `RadioGroup` component in combination with the [Label](/docs/components/label) component.

```rust
use crate::components::ui::label::Label;
use crate::components::ui::radio_button::{RadioGroup, RadioGroupItem};
```

```rust
let value_signal = RwSignal::new("option1".to_string());

view! {
    <RadioGroup value=value_signal>
        <div class="flex gap-3 items-center">
            <RadioGroupItem value="option1" id="option1" />
            <Label html_for="option1">"Option 1"</Label>
        </div>
        <div class="flex gap-3 items-center">
            <RadioGroupItem value="option2" id="option2" />
            <Label html_for="option2">"Option 2"</Label>
        </div>
    </RadioGroup>
}
```




## Examples

### Custom Labels

Radio Group with custom label formatting using `VariantArray` and `IntoStaticStr` from strum for display names.

<StaticRadioButtonCustom />


### Radio Choice Card

Radio group used as choice cards — clicking the entire card selects the option.

<StaticRadioChoiceCard />


## See Also

- [Radio Button Group](/docs/components/radio-button-group)
- [Checkbox](/docs/components/checkbox)
- [Form](/docs/components/form)
- [Label](/docs/components/label)
