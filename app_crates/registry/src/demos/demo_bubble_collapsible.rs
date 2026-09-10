use dioxus::prelude::*;
use icons::ChevronDown;

use crate::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleVariant};
use crate::ui::button::{Button, ButtonVariant};
use crate::ui::collapsible::{Collapsible, CollapsibleTrigger};

const TEXT: &str = "The accessibility review found two focus states that were visually too subtle in dark mode.\n\nI checked the dialog, menu, and drawer paths because each one renders focusable controls inside a layered surface.\n\nThe dialog and drawer are fine. The menu needs the hover and focus tokens split so keyboard focus stays visible when the pointer is not involved.\n\nI also recommend keeping the change in the style file instead of the primitive so the other themes can choose their own focus treatment later.";

const PREVIEW_LEN: usize = 180;

#[component]
pub fn DemoBubbleCollapsible() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        div { class: "flex flex-col gap-8 py-12 w-full max-w-sm",
            Bubble { variant: BubbleVariant::Muted,
                BubbleContent { "How can I help you today?" }
            }
            Bubble { variant: BubbleVariant::Muted, align: BubbleAlign::End,
                BubbleContent { class: "whitespace-pre-line",
                    Collapsible { open: open(),
                        div { {if open() { TEXT } else { &TEXT[..PREVIEW_LEN] } } }
                        CollapsibleTrigger {
                            onclick: move |_| open.set(!open()),
                            Button { variant: ButtonVariant::Link, class: "gap-1 p-0 text-muted-foreground",
                                {if open() { "Show less" } else { "Show more" }}
                                ChevronDown {
                                    class: if open() { "rotate-180 transition-transform" } else { "transition-transform" },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
