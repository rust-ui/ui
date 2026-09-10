use dioxus::prelude::*;

#[component]
pub fn DemoRangeSlider() -> Element {
    rsx! {
        div { class: "p-6 w-full max-w-md bg-white rounded-lg shadow-lg",
            h2 { class: "mb-4 text-2xl font-bold", "Range Slider" }
            div { class: "mb-4",
                label { r#for: "price-range", class: "block mb-2 font-bold text-gray-700",
                    "Price Range"
                }
                input {
                    r#type: "range",
                    id: "price-range",
                    class: "w-full accent-indigo-600",
                    min: "0",
                    max: "1000",
                    value: "500",
                    "oninput": "updatePrice(this.value)",
                }
            }
            div { class: "flex justify-between text-gray-500",
                span { id: "minPrice", "$0" }
                span { id: "maxPrice", "$1000" }
            }
        }

        script { src: "/app_components/range_slider.js" }
    }
}
