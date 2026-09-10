use dioxus::prelude::*;

#[component]
pub fn WorkflowsHero() -> Element {
    rsx! {
        div { class: "flex flex-col gap-2 items-center py-8 text-center md:py-16 lg:py-20 xl:gap-4",
            h1 { class: "max-w-2xl text-4xl font-semibold tracking-tight lg:font-semibold xl:text-5xl xl:tracking-tighter text-primary leading-tighter text-balance lg:leading-[1.1]",
                "Dioxus Workflow Builder"
            }
            p { class: "max-w-3xl text-base sm:text-lg text-foreground text-balance",
                "Interactive node-based workflow components for Dioxus. Drag, connect, and compose workflows — ready to drop into your Rust app."
            }
        }
    }
}
