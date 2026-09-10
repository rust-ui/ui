use dioxus::prelude::*;

#[component]
pub fn TestPage() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6 items-center px-4 mx-auto w-full max-w-[1200px] py-10",
            h1 { class: "text-3xl font-bold", "Test Page" }
            p { class: "text-muted-foreground", "This is a test page." }
        }
    }
}
