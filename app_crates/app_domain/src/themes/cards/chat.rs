use dioxus::prelude::*;
use icons::{Plus, Send};
use registry::ui::avatar::{Avatar, AvatarFallback};
use registry::ui::button::{Button, ButtonSize, ButtonVariant};
use registry::ui::card::{Card, CardContent, CardFooter, CardHeader};
use registry::ui::input::Input;

#[component]
pub fn CardChat() -> Element {
    rsx! {
        Card {
            CardHeader { class: "flex flex-row items-center",
                div { class: "flex items-center space-x-4",
                    Avatar { class: "size-10",
                        AvatarFallback { "SD" }
                    }
                    div {
                        p { class: "text-sm font-medium leading-none", "Sofia Davis" }
                        p { class: "text-sm text-muted-foreground", "m@example.com" }
                    }
                }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Icon,
                    class: "ml-auto rounded-full",
                    Plus {}
                    span { class: "hidden", "New message" }
                }
            }
            CardContent {
                div { class: "space-y-4",
                    ChatBubble { "Hi, how can I help you today?" }
                    ChatBubble { primary: true, "Hey, I'm having trouble with my account." }
                    ChatBubble { "What seems to be the problem?" }
                    ChatBubble { primary: true, "I can not log in." }
                }
            }
            CardFooter {
                form { class: "flex items-center space-x-2 w-full",
                    Input {
                        class: "flex-1",
                        id: "message",
                        placeholder: "Type your message...",
                    }
                    Button { size: ButtonSize::Icon, button_type: "submit", disabled: true,
                        Send {}
                        span { class: "hidden", "Send" }
                    }
                }
            }
        }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn ChatBubble(#[props(default)] primary: bool, children: Element) -> Element {
    let class = if primary {
        "flex flex-col gap-2 py-2 px-3 w-max text-sm rounded-lg max-w-[75%] bg-primary text-primary-foreground ml-auto"
    } else {
        "flex flex-col gap-2 py-2 px-3 w-max text-sm rounded-lg max-w-[75%] bg-muted"
    };
    rsx! {
        div { class: "{class}", {children} }
    }
}
