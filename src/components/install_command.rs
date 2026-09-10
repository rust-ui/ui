use dioxus::prelude::*;
use registry::ui::button::{Button, ButtonVariant};

use crate::components::steps::{Step, Steps};
use crate::markdown::highlight_code::highlight_code;

#[derive(Clone, Copy, PartialEq, Default)]
enum Tab {
    #[default]
    Cli,
    Manual,
}

/// Replaces the "## Installation" stub, matching leptos's `StaticInstallWrapper`
/// exactly: plain-text CLI/Manual tabs (no card chrome), numbered `Step`s.
/// CLI tab shows `ui add <demo_name>` + `ui add <name>`, Manual tab shows the raw
/// component source with expand/collapse.
#[component]
pub fn InstallCommand(
    #[props(into)] name: String,
    #[props(into)] demo_name: String,
    #[props(into)] raw_code: String,
) -> Element {
    let mut tab = use_signal(Tab::default);
    let mut expanded = use_signal(|| false);

    let cli_code = format!("# cargo install ui-cli --force\nui add {demo_name}\nui add {name}");
    let cli_highlighted = highlight_code(&cli_code, Some("bash"), None);
    let manual_highlighted = highlight_code(&raw_code, Some("rust"), None);
    let file_path = format!("app_crates/registry/src/ui/{name}.rs");

    let tab_base = "inline-flex items-center gap-1.5 rounded-sm px-3 py-1.5 text-sm font-medium transition-colors";
    let active = "bg-background text-foreground shadow-sm";
    let inactive = "text-muted-foreground hover:text-foreground";
    let tab_class = move |t: Tab| format!("{tab_base} {}", if tab() == t { active } else { inactive });

    rsx! {
        div { class: "flex flex-col w-full",
            div { class: "self-start inline-flex h-9 items-center rounded-md bg-muted p-1 text-muted-foreground",
                button { class: "{tab_class(Tab::Cli)}", onclick: move |_| tab.set(Tab::Cli), "CLI" }
                button { class: "{tab_class(Tab::Manual)}", onclick: move |_| tab.set(Tab::Manual), "Manual" }
            }

            // CLI tab
            div { style: if tab() == Tab::Cli { "display:block" } else { "display:none" },
                Steps {
                    Step { "You can run either of the following commands:" }
                    pre {
                        class: "bg-muted rounded-xl overflow-x-auto py-3.5 px-4 mt-4 min-w-0 text-xs",
                        dangerous_inner_html: "{cli_highlighted}",
                    }
                    Step { "Update the imports to match your project setup." }
                }
            }

            // Manual tab
            div { style: if tab() == Tab::Manual { "display:block" } else { "display:none" },
                Steps {
                    Step { "Copy and paste the following code into your project:" }
                    p { class: "leading-7 [&:not(:first-child)]:mt-6",
                        code { class: "font-mono rounded bg-muted px-[0.3rem] py-[0.2rem] text-[0.8rem]", "{file_path}" }
                    }
                    div {
                        class: if expanded() { "overflow-auto relative my-6 h-fit" } else { "overflow-hidden relative my-6 h-[100px]" },
                        pre {
                            class: "bg-muted rounded-xl overflow-x-auto py-3.5 px-4 text-xs max-h-[370px]",
                            dangerous_inner_html: "{manual_highlighted}",
                        }
                        Button {
                            variant: ButtonVariant::Secondary,
                            class: "absolute bottom-4 left-1/2 -translate-x-1/2 hover:bg-secondary",
                            onclick: move |_| expanded.set(!expanded()),
                            if expanded() { "Collapse" } else { "Expand" }
                        }
                    }
                    Step { "Update the imports to match your project setup." }
                }
            }
        }
    }
}
