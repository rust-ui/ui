+++
title = "Switch"
description = "Rust/UI component that displays a control that allows the user to toggle between checked and not checked."
tags = ["utils"]
is_new = false
image = "/images/thumbnails/switch.webp"
image_dark = "/images/thumbnails/switch-dark.webp"
+++


<StaticSwitch />






## Installation

<StaticInstallSwitch />








## Usage

```rust
use crate::components::ui::switch::Switch;
```

```rust
<Switch checked=false />
```


### RTL

Toggle switch with an Arabic label. The label appears to the left of the switch, following RTL reading order.

<StaticSwitchRtl />


### Choice Card

Switch used as a choice card — clicking the entire card toggles the switch.

<StaticSwitchChoiceCard />


## See Also

- [Checkbox](/docs/components/checkbox)
- [Form](/docs/components/form)
- [Label](/docs/components/label)
