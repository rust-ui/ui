use dioxus::prelude::*;

use crate::hooks::use_history::{UseHistory, use_history};
use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::kbd::{Kbd, KbdGroup};

const COLORS: &[(&str, &str)] = &[
    ("slate", "bg-slate-500"),
    ("red", "bg-red-500"),
    ("orange", "bg-orange-500"),
    ("green", "bg-green-500"),
    ("blue", "bg-blue-500"),
    ("violet", "bg-violet-500"),
];

#[component]
pub fn DemoUseHistory() -> Element {
    let _ = UseHistory::init();

    rsx! { DemoUseHistoryInner {} }
}

#[component]
fn DemoUseHistoryInner() -> Element {
    let history = use_history();

    let current_url = use_memo(move || history.current());
    let active = use_memo(move || {
        current_url()
            .trim_start_matches('?')
            .split('&')
            .find(|p| p.starts_with("color="))
            .and_then(|p| p.strip_prefix("color="))
            .unwrap_or("slate")
            .to_string()
    });

    let can_back = use_memo(move || history.can_go_back());
    let can_forward = use_memo(move || history.can_go_forward());
    let position = use_memo(move || history.position());
    let total = use_memo(move || history.total());

    rsx! {
        div { class: "flex flex-col gap-6 items-center",

            // Color swatches
            div { class: "flex gap-2 items-center",
                for (name, color_class) in COLORS.iter().copied() {
                    button {
                        class: "size-8 rounded-full transition-all duration-200 {color_class} data-[active=true]:ring-2 data-[active=true]:ring-offset-2 data-[active=true]:ring-current",
                        "data-color": name,
                        "data-active": if active() == name { "true" } else { "false" },
                        onclick: move |_| {
                            history.push(format!("?color={name}"));
                        },
                    }
                }
            }

            // Current state
            p { class: "text-sm text-muted-foreground",
                "State {position()} of {total()} — "
                code { class: "font-mono text-xs", {current_url()} }
            }

            // Undo / Redo buttons
            div { class: "flex gap-2 items-center",
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    disabled: !can_back(),
                    onclick: move |_| history.go_back(),
                    "← Undo"
                }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    disabled: !can_forward(),
                    onclick: move |_| history.go_forward(),
                    "Redo →"
                }
            }

            // Keyboard hints
            div { class: "flex gap-3 items-center text-xs text-muted-foreground",
                span { class: "flex gap-1 items-center",
                    KbdGroup {
                        Kbd { "⌘" }
                        Kbd { "Z" }
                    }
                    " undo"
                }
                span { class: "flex gap-1 items-center",
                    KbdGroup {
                        Kbd { "⌘" }
                        Kbd { "⇧" }
                        Kbd { "Z" }
                    }
                    " redo"
                }
            }
        }
    }
}
