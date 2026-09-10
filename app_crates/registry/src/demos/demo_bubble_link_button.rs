use dioxus::prelude::*;

use crate::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleGroup, BubbleVariant};

#[component]
pub fn DemoBubbleLinkButton() -> Element {
    rsx! {
        div { class: "flex flex-col gap-8 py-12 w-full max-w-sm",
            Bubble { variant: BubbleVariant::Muted,
                BubbleContent { "How can I help you today?" }
            }
            BubbleGroup {
                Bubble { variant: BubbleVariant::Tinted, align: BubbleAlign::End,
                    BubbleContent {
                        onclick: move |_| {
                            let _ = web_sys::window()
                                .and_then(|w| w.alert_with_message("You clicked forgot password").ok());
                        },
                        "I forgot my password"
                    }
                }
                Bubble { variant: BubbleVariant::Tinted, align: BubbleAlign::End,
                    BubbleContent {
                        onclick: move |_| {
                            let _ = web_sys::window()
                                .and_then(|w| w.alert_with_message("You clicked help with subscription").ok());
                        },
                        "I need help with my subscription"
                    }
                }
                Bubble { variant: BubbleVariant::Tinted, align: BubbleAlign::End,
                    BubbleContent {
                        onclick: move |_| {
                            let _ = web_sys::window()
                                .and_then(|w| {
                                    w.alert_with_message("You clicked something else. Talk to a human.").ok()
                                });
                        },
                        "Something else. Talk to a human."
                    }
                }
            }
        }
    }
}
