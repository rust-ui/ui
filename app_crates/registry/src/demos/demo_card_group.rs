use dioxus::prelude::*;
use icons::{CircleAlert, Cloud, Search};

use crate::ui::button::{Button, ButtonVariant};

#[component]
pub fn DemoCardGroup() -> Element {
    rsx! {
        div { class: "group p-14 w-full max-w-[620px] text-center rounded-xl border transition duration-500 hover:duration-200 bg-accent hover:bg-secondary",
            div { class: "flex justify-center isolate",
                div { class: "grid place-items-center bg-white rounded-xl ring-1 transition duration-500 group-hover:duration-200 size-12 ring-black/[0.08] relative top-1.5 left-2.5 -rotate-6 group-hover:-rotate-12 group-hover:-translate-x-5 shadow-lg",
                    Cloud { class: "size-4" }
                }
                div { class: "grid place-items-center bg-white rounded-xl ring-1 transition duration-500 group-hover:duration-200 size-12 ring-black/[0.08] z-10 shadow-lg",
                    Search { class: "size-4" }
                }
                div { class: "grid place-items-center bg-white rounded-xl ring-1 transition duration-500 group-hover:duration-200 size-12 ring-black/[0.08] relative top-1.5 right-2.5 rotate-6 group-hover:rotate-12 group-hover:translate-x-5 shadow-lg",
                    CircleAlert { class: "size-4" }
                }
            }
            h2 { class: "mt-6 text-base font-medium", "No Icons Found" }
            p { class: "mx-auto mt-1 text-sm text-muted-foreground max-w-[300px]",
                "You were searching for Icons in Rust/UI but none of them was found. Sorry!"
            }
            div { class: "mt-4 flex justify-center",
                Button { variant: ButtonVariant::Outline, href: "/icons", "Go Back to Icons Page" }
            }
        }
    }
}
