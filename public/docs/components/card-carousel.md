+++
title = "Card Carousel"
description = "Rust/UI component that displays a card similar as Airbnb Card."
tags = ["card"]
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

<StaticCardCarousel />

## Installation

<StaticInstallCardCarousel />

## Usage

```rust
use crate::ui::card_carousel::{CardCarousel, CardCarouselTrack, CardCarouselSlide, CardCarouselImage, CardCarouselOverlay, CardCarouselNav, CardCarouselNavButton, CardCarouselIndicators, CardCarouselIndicator};
```

```rust
rsx! {
    CardCarousel {
        CardCarouselOverlay {
            CardCarouselNav {
                CardCarouselNavButton { ChevronLeft {} }
                CardCarouselNavButton { ChevronRight {} }
            }
            CardCarouselIndicators {
                CardCarouselIndicator { aria_current: true }
                CardCarouselIndicator {}
            }
        }
        CardCarouselTrack {
            CardCarouselSlide {
                CardCarouselImage { src: "/img1.jpg", alt: "Slide 1" }
            }
        }
    }
}
```

## See Also

- [Carousel](/components/carousel)
- [Card](/components/card)
