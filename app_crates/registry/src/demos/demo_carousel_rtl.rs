use dioxus::prelude::*;

use crate::ui::carousel::{Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious};
use crate::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoCarouselRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "px-12 mx-auto w-full max-w-xs",
            Carousel { looping: true,
                CarouselContent {
                    for i in 1u32..=5 {
                        CarouselItem {
                            div { class: "p-1",
                                div { class: "flex justify-center items-center p-6 rounded-lg border aspect-square bg-card shadow-xs",
                                    span { class: "text-4xl font-semibold text-foreground", "{i}" }
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
