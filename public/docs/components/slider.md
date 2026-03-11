+++
title = "Slider"
description = "Rust/UI component that allows users to select a value from a range."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++



<StaticSlider />








## Installation

<StaticInstallSlider />




## Usage

```rust
use crate::components::ui::slider::Slider;
```

```rust
<Slider attr:min="0" attr:max="100" attr:value="40" attr:step="5" />
```



## Examples

### Flat

Slider component with flat styling for minimal visual design. This example demonstrates how to create sleek range sliders in Leptos with reduced shadows and borders for modern, understated UI aesthetics in Rust applications.

<StaticSliderFlat />


### Vertical

Vertical slider orientation using CSS rotation, available in both Round and Flat variants. Ideal for volume controls, equalizers, or any UI where a vertical range input fits the layout.

<StaticSliderVertical />


### Multiple

Three independent sliders representing separate values — useful for multi-band controls like equalizers, color channels, or any scenario requiring multiple distinct range inputs.

<StaticSliderMultiple />


### Controlled

Controlled slider using a Leptos reactive signal. The displayed value updates in real time as the slider moves, demonstrating how to bind range input state in Rust applications.

<StaticSliderControlled />


### RTL

Slider in RTL mode. The track fills from right to left and the thumb travels in the right-to-left direction.

<StaticSliderRtl />

## See Also

- [Form](/docs/components/form)
- [Input](/docs/components/input)
