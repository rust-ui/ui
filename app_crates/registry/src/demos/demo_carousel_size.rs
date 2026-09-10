use dioxus::prelude::*;

use crate::ui::carousel::{Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious};

#[component]
pub fn DemoCarouselSize() -> Element {
    rsx! {
        div { class: "px-12 mx-auto w-full max-w-sm",
            Carousel {
                CarouselContent {
                    for i in 1u32..=5 {
                        CarouselItem { class: "md:basis-1/2 lg:basis-1/3",
                            div { class: "p-1",
                                div { class: "flex justify-center items-center p-6 rounded-lg border aspect-square bg-card shadow-xs",
                                    span { class: "text-3xl font-semibold text-foreground", "{i}" }
                                }
                            }
                        }
                    }
                }
                CarouselPrevious {}
                CarouselNext {}
            }
        }
    }
}
