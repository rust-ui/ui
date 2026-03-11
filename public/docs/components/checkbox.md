+++
title = "Checkbox"
description = "Rust/UI component that displays a control that allows the user to toggle between checked and not checked."
tags = ["utils"]
is_new = false
image = "/images/thumbnails/checkbox.webp"
image_dark = "/images/thumbnails/checkbox-dark.webp"
+++


<StaticCheckbox />







## Installation

<StaticInstallCheckbox />






## Usage

You can use the `Checkbox` component in combination with the [Label](/docs/components/label) component.

```rust
use crate::components::ui::checkbox::Checkbox;
```

```rust
<Checkbox label="Accept terms and conditions" />
```


### RTL

Checkbox items with Arabic labels. The checkbox appears on the right and the label on the left, matching right-to-left reading flow.

<StaticCheckboxRtl />

## See Also

- [Form](/docs/components/form)
- [Label](/docs/components/label)
- [Switch](/docs/components/switch)
- [Radio Button](/docs/components/radio-button)
