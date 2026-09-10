use dioxus::prelude::*;
use icons::{ChevronLeft, ChevronRight};

use crate::ui::card::{CardContent, CardDescription, CardTitle};
use crate::ui::card_carousel::{
    CardCarousel, CardCarouselImage, CardCarouselIndicator, CardCarouselIndicators, CardCarouselNav,
    CardCarouselNavButton, CardCarouselOverlay, CardCarouselSlide, CardCarouselTrack,
};

const IMAGES: &[&str] = &[
    "https://a0.muscache.com/im/pictures/e24c13b9-dd2a-4e15-9845-dd588a884e39.jpg?im_w=720",
    "https://a0.muscache.com/im/pictures/373443ec-b377-4181-b753-3a2f3508c2b3.jpg?im_w=720",
    "https://a0.muscache.com/im/pictures/97b92645-f975-4e60-9ae0-205885af64b0.jpg?im_w=720",
    "https://a0.muscache.com/im/pictures/0089340a-409a-4fbe-9ab7-cb26884bf267.jpg?im_w=720",
    "https://a0.muscache.com/im/pictures/90e58fdf-257b-43a5-a6dc-a08f518397fe.jpg?im_w=720",
];

#[component]
pub fn DemoCardCarousel() -> Element {
    let images_count = IMAGES.len();

    rsx! {
        div { class: "my-4",
            CardCarousel {
                CardCarouselOverlay {
                    CardCarouselNav {
                        CardCarouselNavButton { aria_disabled: true, ChevronLeft {} }
                        CardCarouselNavButton { ChevronRight {} }
                    }
                    CardCarouselIndicators {
                        for i in 0..images_count {
                            CardCarouselIndicator { aria_current: i == 0 }
                        }
                    }
                }
                CardCarouselTrack {
                    for src in IMAGES {
                        CardCarouselSlide {
                            CardCarouselImage { src: *src, alt: "CardCarousel img" }
                        }
                    }
                }
            }
            CardContent { class: "py-4",
                CardTitle { "MV, Maldives" }
                CardDescription { "4,843 kilometers away" }
                CardDescription { "Aug 1 – 6" }
                CardDescription { "$685 per night" }
            }
        }
    }
}
