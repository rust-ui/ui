use dioxus::prelude::*;

#[component]
pub fn TestServerFunctions() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gray-950 text-white flex flex-col items-center justify-center",
            p { class: "text-gray-400", "Server functions are not available in static builds." }
        }
    }
}
