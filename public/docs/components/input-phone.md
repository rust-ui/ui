+++
title = "Input Phone"
description = "Rust/UI component that displays a phone number input with country code selector and automatic formatting."
tags = ["input"]
is_new = true
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++


<StaticInputPhone />




## Installation

<StaticInstallInputPhone />



## Usage

```rust
use crate::components::ui::input_phone::InputPhone;
use crate::utils::country::Country;
use crate::utils::phone_number::PhoneNumber;
```

```rust
let phone_signal = RwSignal::new(PhoneNumber::default());
let country_signal = RwSignal::new(Country::UnitedStatesOfAmerica);

view! {
    <InputPhone value_signal=phone_signal country_signal=country_signal />
}
```



## Examples

### Disabled

A disabled phone input with pre-filled value.

<StaticInputPhoneDisabled />



## See Also

- [Input](/docs/components/input)
- [Select](/docs/components/select)
- [Form](/docs/components/form)
