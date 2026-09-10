use dioxus::prelude::*;
use icons::Calendar;

use crate::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::ui::button::{Button, ButtonVariant};
use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::hover_card::{HoverCard, HoverCardContent, HoverCardTrigger};

#[component]
pub fn DemoHoverCardRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "max-w-fit",
            HoverCard {
                HoverCardTrigger {
                    Button { variant: ButtonVariant::Link, "@rust-lang" }
                }
                HoverCardContent { class: "w-80",
                    div { class: "flex gap-4",
                        Avatar {
                            AvatarImage { src: "https://api.dicebear.com/9.x/notionists/svg?seed=rust-lang", alt: "@rust-lang" }
                            AvatarFallback { "RL" }
                        }
                        div { class: "flex flex-col gap-1",
                            h4 { class: "text-sm font-semibold", "@rust-lang" }
                            p { class: "text-sm text-muted-foreground",
                                "تمكين الجميع من بناء برامج موثوقة وفعّالة."
                            }
                            div { class: "flex gap-2 items-center mt-2",
                                Calendar { class: "size-3.5 text-muted-foreground" }
                                span { class: "text-xs text-muted-foreground", "نشط منذ ٢٠١٠" }
                            }
                        }
                    }
                }
            }
        }
    }
}
