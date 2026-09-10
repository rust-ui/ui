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

## Usage

```rust
use crate::ui::carousel::{Carousel, CarouselContent, CarouselItem, CarouselPrevious, CarouselNext};
```

```rust
rsx! {
    Carousel { looping: true,
        CarouselContent {
            CarouselItem {
                div { class: "p-1",
                    Card { "Slide 1" }
                }
            }
            CarouselItem {
                div { class: "p-1",
                    Card { "Slide 2" }
                }
            }
        }
        CarouselPrevious {}
        CarouselNext {}
    }
}
```

## See Also

- [Card Carousel](/components/card-carousel)
- [Scroll Area](/components/scroll-area)
