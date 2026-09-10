use app_domain::themes::theme_name::ThemeName;
use dioxus::prelude::*;

#[component]
pub fn ThemePicker(mut theme: Signal<ThemeName>) -> Element {
    rsx! {
        div { class: "space-y-2",
            small { class: "text-sm font-medium leading-none", "Base Color" }
            div { class: "flex flex-col gap-1",
                {ThemeName::ALL.iter().map(|&t| {
                    let is_active = move || theme() == t;
                    rsx! {
                        button {
                            key: "{t.label()}",
                            class: "flex gap-2 items-center py-1.5 px-3 w-full text-sm text-left rounded-md transition-colors hover:bg-accent hover:text-accent-foreground",
                            class: if is_active() { "bg-accent text-accent-foreground font-medium" },
                            onclick: move |_| theme.set(t),
                            span {
                                class: "inline-block flex-shrink-0 rounded-full border size-4 border-border/50",
                                style: "background-color:{t.swatch()}"
                            }
                            "{t.label()}"
                        }
                    }
                })}
            }
        }
    }
}
