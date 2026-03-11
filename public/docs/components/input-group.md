+++
title = "Input Group"
description = "A component that combines inputs with addons like icons, text, or buttons."
tags = ["input"]
is_new = true
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



## Usage

```rust
use icons::Search;

use crate::components::ui::input_group::{InputGroup, InputGroupAddon, InputGroupInput};
```

```rust
<InputGroup>
    <InputGroupInput attr:placeholder="Search..." />
    <InputGroupAddon>
        <Search />
    </InputGroupAddon>
</InputGroup>
```




## Examples

### Input Group Text

Input field combined with text addons for labels, units, or prefixes. This example demonstrates how to build enhanced [Input](/docs/components/input) components in Leptos using InputGroupAddon for displaying contextual text and improving form usability in Rust applications.

<StaticInputGroupText />


### AI Prompt

Block-aligned addons for a vertical layout — ideal for AI chat inputs, rich text editors, or prompt builders.

<StaticInputGroupBlock />


### RTL

Input groups with Arabic placeholders. Leading and trailing addons swap sides so icons remain visually anchored in RTL.

<StaticInputGroupRtl />

## See Also

- [Input](/docs/components/input)
- [Button](/docs/components/button)
- [Label](/docs/components/label)
