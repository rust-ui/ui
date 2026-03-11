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



## Components

The CardCarousel component is composed of several subcomponents:

- **CardCarousel**: Main wrapper component managing carousel state
- **CardCarouselTrack**: Sliding container for carousel slides
- **CardCarouselSlide**: Individual slide container
- **CardCarouselImage**: Image element within a slide
- **CardCarouselOverlay**: Overlay layer for controls
- **CardCarouselNav**: Navigation controls container
- **CardCarouselNavButton**: Previous/next navigation buttons
- **CardCarouselIndicators**: Container for slide indicators
- **CardCarouselIndicator**: Individual slide position indicator



## Usage

```rust
use crate::components::ui::card_carousel::{
    CardCarousel,
    CardCarouselImage,
    CardCarouselIndicator,
    CardCarouselIndicators,
    CardCarouselNav,
    CardCarouselNavButton,
    CardCarouselOverlay,
    CardCarouselSlide,
    CardCarouselTrack,
};
```

```rust
<CardCarousel>
    <CardCarouselTrack>
        <CardCarouselSlide>
            <CardCarouselImage attr:src="/image1.jpg" attr:alt="Image 1" />
        </CardCarouselSlide>
        <CardCarouselSlide>
            <CardCarouselImage attr:src="/image2.jpg" attr:alt="Image 2" />
        </CardCarouselSlide>
    </CardCarouselTrack>
    <CardCarouselOverlay>
        <CardCarouselNav>
            <CardCarouselNavButton>"←"</CardCarouselNavButton>
            <CardCarouselNavButton>"→"</CardCarouselNavButton>
        </CardCarouselNav>
        <CardCarouselIndicators>
            <CardCarouselIndicator />
            <CardCarouselIndicator />
        </CardCarouselIndicators>
    </CardCarouselOverlay>
</CardCarousel>
```


## See Also

- [Card](/docs/components/card)
