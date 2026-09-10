use dioxus::prelude::*;
use icons::FileQuestion;

#[component]
pub fn DemoDocker() -> Element {
    let button_titles = vec!["Settings", "Browser", "Mail", "Map", "Messages", "Music Player", "Apps", "Documents"];

    rsx! {
        link { rel: "stylesheet", href: "/app_components/docker.css" }

        div { class: "h-[600px]",
            h1 { class: "text-3xl font-bold lg:text-4xl text-pretty",
                "Dock magnification using "
                code { ":has()" }
            }
            nav { class: "flex fixed gap-1 justify-center items-end dockerNav",
                {button_titles.into_iter().map(|title| rsx! {
                    button {
                        r#type: "button",
                        class: "relative border-none transition-all duration-300 ease-in-out cursor-pointer outline-hidden bg-[rgba(0,0,0,0.75)] text-[rgba(215,255,255,1)] w-[var(--btn-width,var(--btn-size))] h-[var(--btn-height,var(--btn-size))] aspect-ratio-1 rounded-[calc(var(--btn-size)*.25)]",
                        "data-title": title,
                        FileQuestion { class: "size-14" }
                    }
                })}
            }
        }
    }
}
