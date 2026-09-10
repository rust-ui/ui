use dioxus::prelude::*;

#[component]
pub fn DemoRangeSliderDual() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "/app_components/range_slider_dual.css" }
        script { src: "/app_components/range_slider_dual.js" }

        div { class: "flex justify-center items-center w-full bg-gray-100 h-[300px]",
            div { class: "p-6 bg-white rounded-lg shadow-lg w-[400px]",
                h2 { class: "text-lg font-bold", "PRICE RANGE" }

                div { class: "relative mt-4 slider-container",
                    input { r#type: "range", id: "minRange", min: "0", max: "400", value: "0" }
                    input { r#type: "range", id: "maxRange", min: "0", max: "400", value: "400" }

                    div { class: "relative w-full h-2 bg-gray-200 rounded-md",
                        div {
                            id: "rangeTrack",
                            class: "absolute h-2 bg-gradient-to-r from-blue-900 to-blue-400 rounded-md",
                        }
                    }
                }

                div { class: "flex justify-between mt-3 text-gray-600",
                    span { "Min Price: $" span { id: "minValue", "0" } }
                    span { "Max Price: $" span { id: "maxValue", "400" } }
                }
            }
        }
    }
}
