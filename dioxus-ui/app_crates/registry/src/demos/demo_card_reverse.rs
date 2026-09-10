use dioxus::prelude::*;

use crate::ui::card::{Card, CardDescription, CardTitle};

#[component]
pub fn DemoCardReverse() -> Element {
    rsx! {
        div { class: "space-y-6",
            Card { class: "flex flex-col gap-6 px-6 md:flex-row",
                div { class: "w-full h-48 rounded-lg md:w-1/3 md:h-32 bg-muted" }
                div { class: "flex-1 pt-0",
                    CardTitle { class: "mb-3", "Nature's Beauty" }
                    CardDescription {
                        "Nature's beauty encompasses a vast array of colors, sounds, and textures that evoke a sense of wonder. Its rhythms and patterns create a calming atmosphere that can rejuvenate the spirit."
                    }
                }
            }
            Card { class: "flex flex-col gap-6 px-6 md:flex-row-reverse",
                div { class: "w-full h-48 rounded-lg md:w-1/3 md:h-32 bg-accent" }
                div { class: "flex-1 pt-0",
                    CardTitle { class: "mb-3", "Ecosystem Balance" }
                    CardDescription {
                        "The intricate balance of ecosystems showcases the interdependence of all living beings. Each element, from the smallest insect to the largest tree, plays a vital role in sustaining life."
                    }
                }
            }
            Card { class: "flex flex-col gap-6 px-6 md:flex-row",
                div { class: "w-full h-48 rounded-lg md:w-1/3 md:h-32 bg-muted" }
                div { class: "flex-1 pt-0",
                    CardTitle { class: "mb-3", "Changing Landscapes" }
                    CardDescription {
                        "The ever-changing landscapes of nature remind us of the passage of time. Seasons bring transformations that create a dynamic environment filled with diverse flora and fauna."
                    }
                }
            }
        }
    }
}
