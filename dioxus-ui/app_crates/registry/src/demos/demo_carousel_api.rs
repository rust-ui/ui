use dioxus::prelude::*;

use crate::ui::carousel::{Carousel, CarouselContent, CarouselIndicator, CarouselItem, CarouselNext, CarouselPrevious};

#[component]
pub fn DemoCarouselApi() -> Element {
    rsx! {
        div { class: "px-12 mx-auto w-full max-w-xs",
            Carousel {
                CarouselContent {
                    for i in 1u32..=5 {
                        CarouselItem {
                            div { class: "flex justify-center items-center p-6 rounded-lg border aspect-square bg-card shadow-xs",
                                span { class: "text-4xl font-semibold text-foreground", "{i}" }
                            }
                        }
                    }
                }
                CarouselPrevious {}
                CarouselNext {}
                CarouselIndicator {}
            }
        }
    }
}
