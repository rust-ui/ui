---
title: "Demo Card Carousel"
name: "demo_card_carousel"
cargo_dependencies: []
registry_dependencies: ["card", "card_carousel"]
type: "components:demos"
path: "demos/demo_card_carousel.rs"
---

# Demo Card Carousel

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_card_carousel
```

## Component Code

```rust
use icons::{ChevronLeft, ChevronRight};
use leptos::prelude::*;

use crate::components::ui::card::{CardContent, CardDescription, CardTitle};
use crate::components::ui::card_carousel::{
    CardCarousel, CardCarouselImage, CardCarouselIndicator, CardCarouselIndicators, CardCarouselNav,
    CardCarouselNavButton, CardCarouselOverlay, CardCarouselSlide, CardCarouselTrack,
};

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn DemoCardCarousel() -> impl IntoView {
    let images_count = IMAGES.len();

    view! {
        <div class="my-4">
            <CardCarousel>
                <CardCarouselOverlay>
                    <CardCarouselNav>
                        <CardCarouselNavButton attr:aria-disabled=true>
                            <ChevronLeft />
                        </CardCarouselNavButton>
                        <CardCarouselNavButton>
                            <ChevronRight />
                        </CardCarouselNavButton>
                    </CardCarouselNav>
                    <CardCarouselIndicators>
                        {(0..images_count)
                            .map(|i| {
                                view! { <CardCarouselIndicator attr:aria-current=move || i == 0 /> }
                            })
                            .collect_view()}
                    </CardCarouselIndicators>
                </CardCarouselOverlay>
                <CardCarouselTrack>
                    {IMAGES
                        .iter()
                        .map(|src| {
                            view! {
                                <CardCarouselSlide>
                                    <CardCarouselImage attr:src=*src attr:alt="CardCarousel img" />
                                </CardCarouselSlide>
                            }
                        })
                        .collect_view()}
                </CardCarouselTrack>
            </CardCarousel>

            <CardContent class="py-4">
                <CardTitle>"MV, Maldives"</CardTitle>
                <CardDescription>"4,843 kilometers away"</CardDescription>
                <CardDescription>"Aug 1 – 6"</CardDescription>
                <CardDescription>"$685 per night"</CardDescription>
            </CardContent>
        </div>

        <script type="module" src="/components/card_carousel.js"></script>
    }
}

/* ========================================================== */
/*                     ✨ CONSTANTS ✨                        */
/* ========================================================== */

const IMAGES: &[&str] = &[
    "https://a0.muscache.com/im/pictures/e24c13b9-dd2a-4e15-9845-dd588a884e39.jpg?im_w=720",
    "https://a0.muscache.com/im/pictures/373443ec-b377-4181-b753-3a2f3508c2b3.jpg?im_w=720",
    "https://a0.muscache.com/im/pictures/97b92645-f975-4e60-9ae0-205885af64b0.jpg?im_w=720",
    "https://a0.muscache.com/im/pictures/0089340a-409a-4fbe-9ab7-cb26884bf267.jpg?im_w=720",
    "https://a0.muscache.com/im/pictures/90e58fdf-257b-43a5-a6dc-a08f518397fe.jpg?im_w=720",
];
```
