+++
title = "Carousel"
description = "Rust/UI component for cycling through elements — slides, images, or cards — with prev/next navigation."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++



<StaticCarousel />




## Installation

<StaticInstallCarousel />




## Components

The Carousel component is composed of several subcomponents:

- **Carousel**: Root wrapper with orientation, loop, and keyboard navigation
- **CarouselContent**: Scrollable slide container
- **CarouselItem**: Individual slide wrapper — use `class="basis-1/2"` for multiple visible items
- **CarouselPrevious**: Previous slide button (absolute positioned)
- **CarouselNext**: Next slide button (absolute positioned)
- **CarouselIndicator**: Displays current slide count (e.g. "2 / 5")



## Usage

```rust
use crate::components::ui::carousel::{
    Carousel,
    CarouselContent,
    CarouselItem,
    CarouselNext,
    CarouselPrevious,
};
```

```rust
<Carousel>
    <CarouselContent>
        <CarouselItem>"Slide 1"</CarouselItem>
        <CarouselItem>"Slide 2"</CarouselItem>
        <CarouselItem>"Slide 3"</CarouselItem>
    </CarouselContent>
    <CarouselPrevious />
    <CarouselNext />
</Carousel>
```




## Examples


### Size

Multiple items visible at once using responsive `basis-*` classes on `CarouselItem`. This example shows how to display partial slides in Leptos for building image galleries, product grids, or content sliders in Rust applications.

<StaticCarouselSize />


### Spacing

Custom item spacing by overriding the default gap on `CarouselContent` and `CarouselItem`. This example demonstrates fine-grained layout control in Leptos for compact or airy carousel designs in Rust applications.

<StaticCarouselSpacing />


### Orientation

Vertical carousel using `orientation=CarouselOrientation::Vertical` with a fixed height on `CarouselContent`. Ideal for vertical sliders, timeline views, or stacked content in Rust/Leptos applications.

<StaticCarouselOrientation />


### API

Slide counter using `CarouselIndicator` that displays the current position automatically. This example demonstrates how to show "Slide X of Y" feedback in Leptos carousels without external state management in Rust applications.

<StaticCarouselApi />



### RTL

Carousel in RTL mode. Previous and Next navigation arrows swap their visual positions to match the right-to-left reading direction.

<StaticCarouselRtl />

## See Also

- [Card](/docs/components/card)
- [Scroll Area](/docs/components/scroll-area)
