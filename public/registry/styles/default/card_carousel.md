---
title: "Card Carousel"
name: "card_carousel"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/card_carousel.rs"
---

# Card Carousel

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add card_carousel
```

## Component Code

```rust
use leptos::prelude::*;
use leptos_ui::{clx, void};

mod components {
    use super::*;
    clx! {CardCarousel, div, "group rounded-[20px] overflow-hidden relative w-[320px] h-[320px] bg-gray-200"}
    clx! {CardCarouselOverlay, div, "pb-4 absolute bottom-0 flex flex-col justify-between items-center z-10 h-[calc(50%+32px)] w-full"}
    clx! {CardCarouselNav, div, "opacity-0 invisible group-hover:visible group-hover:opacity-100 transition-opacity duration-[240ms] p-3 flex justify-between items-center w-full"}
    clx! {CardCarouselNavButton, button, "border-0 rounded-full cursor-pointer flex items-center justify-center size-8 [&_svg:not([class*='size-'])]:size-3 bg-accent transition-all duration-[160ms] ease-in-out hover:shadow-sm hover:scale-110 aria-[disabled]:invisible"}
    clx! {CardCarouselIndicators, div, "gap-1 flex"}
    void! {CardCarouselIndicator, span, "rounded-full size-[6px] bg-white opacity-60 aria-[current]:opacity-100"}
    clx! {CardCarouselSlide, div, "snap-center shrink-0 w-full h-full"}
    void! {CardCarouselImage, img, "object-cover w-full h-full"}
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn CardCarouselTrack(children: Children) -> impl IntoView {
    view! {
        <div
            data-name="CardCarouselTrack"
            class="flex overflow-x-scroll w-full h-full snap-x snap-mandatory scroll-smooth touch-pan-x [scrollbar-width:none] [&::-webkit-scrollbar]:hidden"
        >
            {children()}
        </div>
    }
}
```
