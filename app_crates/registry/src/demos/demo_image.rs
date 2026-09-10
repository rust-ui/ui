use dioxus::prelude::*;

use crate::ui::image::Image;

#[component]
pub fn DemoImage() -> Element {
    rsx! {
        Image {
            src: "https://images.unsplash.com/photo-1588345921523-c2dcdb7f1dcd?w=800&dpr=2&q=80",
            alt: "A photo",
            width: 400,
            height: 300,
            class: "rounded-lg object-cover",
        }
    }
}
