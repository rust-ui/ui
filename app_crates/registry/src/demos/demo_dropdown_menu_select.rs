use dioxus::prelude::*;
use icons::{ChevronsUpDown, LayoutTemplate, Sparkles, Star};

use crate::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAlign, DropdownMenuContent, DropdownMenuRadioGroup, DropdownMenuRadioItem,
    DropdownMenuTrigger,
};

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum Registry {
    #[default]
    Components,
    Icons,
    Extensions,
}

impl Registry {
    fn label(self) -> &'static str {
        match self {
            Registry::Components => "Components",
            Registry::Icons => "Icons",
            Registry::Extensions => "Extensions",
        }
    }
}

#[component]
pub fn DemoDropdownMenuSelect() -> Element {
    let registry_signal = use_signal(Registry::default);

    rsx! {
        DropdownMenu { align: DropdownMenuAlign::Center,
            DropdownMenuTrigger { class: "flex justify-between px-2 w-full h-12 bg-transparent border-0",
                div { class: "flex gap-2 items-center",
                    div { class: "flex justify-center items-center rounded-lg bg-primary text-primary-foreground aspect-square size-8",
                        match registry_signal() {
                            Registry::Components => rsx! { LayoutTemplate {} },
                            Registry::Icons => rsx! { Star {} },
                            Registry::Extensions => rsx! { Sparkles {} },
                        }
                    }

                    div { class: "grid flex-1 text-sm leading-tight text-left",
                        span { class: "font-medium", "Registry" }
                        span { class: "text-xs", "{registry_signal().label()}" }
                    }
                }

                ChevronsUpDown {}
            }

            DropdownMenuContent {
                DropdownMenuRadioGroup { value: registry_signal,
                    DropdownMenuRadioItem { value: Registry::Components,
                        LayoutTemplate { class: "text-muted-foreground" }
                        "Components"
                    }
                    DropdownMenuRadioItem { value: Registry::Icons,
                        Star { class: "text-muted-foreground" }
                        "Icons"
                    }
                    DropdownMenuRadioItem { value: Registry::Extensions,
                        Sparkles { class: "text-muted-foreground" }
                        "Extensions"
                    }
                }
            }
        }
    }
}
