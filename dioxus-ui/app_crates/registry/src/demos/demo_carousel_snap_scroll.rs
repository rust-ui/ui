use dioxus::prelude::*;

#[component]
pub fn DemoCarouselSnapScroll() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "/app_components/carousel-snap-scroll.css" }

        div { class: "mainDiv",
            div { class: "scrollsnap-carousel",
                div { class: "slide",
                    div { class: "content",
                        div { class: "content-wrapper", "slide one" }
                    }
                }
                div { class: "slide",
                    div { class: "content",
                        div { class: "content-wrapper", "slide two" }
                    }
                }
                div { class: "slide",
                    div { class: "content",
                        div { class: "content-wrapper", "slide three" }
                    }
                }
                div { class: "slide",
                    div { class: "content",
                        div { class: "content-wrapper", "slide four" }
                    }
                }
                div { class: "slide",
                    div { class: "content",
                        div { class: "content-wrapper", "slide five" }
                    }
                }
                div { class: "slide",
                    div { class: "content",
                        div { class: "content-wrapper", "slide six" }
                    }
                }
                div { class: "slide",
                    div { class: "content",
                        div { class: "content-wrapper", "slide seven" }
                    }
                }
                div { class: "slide",
                    div { class: "content",
                        div { class: "content-wrapper", "slide eight" }
                    }
                }
            }
        }
    }
}
