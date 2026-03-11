


```rust

use icons::Send;
use leptos::prelude::*;

use crate::components::ui::avatar::{Avatar, AvatarImage};
use crate::components::ui::button::Button;
use crate::components::ui::chat::{
    ChatBody, ChatCard, ChatFooter, ChatHeader, ChatMessageBubble, ChatMessageContent, ChatMessageList,
    ChatMessageReceived, ChatMessageSent, ChatMessageTime,
};
use crate::components::ui::input::Input;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn Chat01() -> impl IntoView {
    view! {
        <section class="container py-16 mx-auto">
            <ChatCard class="mx-auto max-w-sm shadow-sm bg-card text-card-foreground h-[70vh] max-h-[700px] min-h-[600px]">
                <ChatHeader class="flex-row p-4 space-y-1.5">
                    <div class="flex items-center space-x-4">
                        <Avatar class="ring-2 ring-offset-2 size-10 ring-primary ring-offset-background">
                            <AvatarImage
                                attr:alt="The Joker"
                                attr:src="https://randomuser.me/api/portraits/men/61.jpg"
                            />
                        </Avatar>
                        <div>
                            <p class="text-base font-medium leading-none">"The Joker"</p>
                            <p class="text-sm text-muted-foreground">"Online"</p>
                        </div>
                    </div>
                </ChatHeader>
                <ChatBody class="p-0">
                    <div
                        dir="ltr"
                        class="overflow-hidden relative p-4 w-full h-full"
                        style="position: relative; --radix-scroll-area-corner-width: 0px; --radix-scroll-area-corner-height: 0px;"
                    >
                        <div
                            data-radix-scroll-area-viewport=""
                            class="w-full h-full rounded-[inherit]"
                            style="overflow: hidden scroll;"
                        >
                            <div style="min-width: 100%; display: table;">
                                <ChatMessageList class="space-y-5">
                                    <ChatMessageReceived class="justify-start items-end mr-auto space-x-2">
                                        <Avatar>
                                            <AvatarImage
                                                attr:alt="The Joker"
                                                attr:src="https://randomuser.me/api/portraits/men/61.jpg"
                                            />
                                        </Avatar>
                                        <ChatMessageBubble class="py-2.5 px-4 shadow-sm bg-muted">
                                            <ChatMessageContent>"Hello! How can I help you today?"</ChatMessageContent>
                                            <ChatMessageTime class="mt-1.5 text-muted-foreground/70">
                                                "14:01"
                                            </ChatMessageTime>
                                        </ChatMessageBubble>
                                    </ChatMessageReceived>
                                    <ChatMessageSent class="justify-end items-end space-x-2">
                                        <ChatMessageBubble class="py-2.5 px-4 shadow-sm bg-primary text-primary-foreground">
                                            <ChatMessageContent>
                                                "I need to reset my password, but I'm not receiving the email."
                                            </ChatMessageContent>
                                            <ChatMessageTime class="mt-1.5 text-primary-foreground/70">
                                                "14:02"
                                            </ChatMessageTime>
                                        </ChatMessageBubble>
                                    </ChatMessageSent>
                                    <ChatMessageReceived class="justify-start items-end mr-auto space-x-2">
                                        <Avatar>
                                            <AvatarImage
                                                attr:alt="The Joker"
                                                attr:src="https://randomuser.me/api/portraits/men/61.jpg"
                                            />
                                        </Avatar>
                                        <ChatMessageBubble class="py-2.5 px-4 shadow-sm bg-muted">
                                            <ChatMessageContent>
                                                "Okay, please provide the email address associated with your account."
                                            </ChatMessageContent>
                                            <ChatMessageTime class="mt-1.5 text-muted-foreground/70">
                                                "14:03"
                                            </ChatMessageTime>
                                        </ChatMessageBubble>
                                    </ChatMessageReceived>
                                    <ChatMessageSent class="justify-end items-end space-x-2">
                                        <ChatMessageBubble class="py-2.5 px-4 shadow-sm bg-primary text-primary-foreground">
                                            <ChatMessageContent>"It is test@example.com"</ChatMessageContent>
                                            <ChatMessageTime class="mt-1.5 text-primary-foreground/70">
                                                "14:03"
                                            </ChatMessageTime>
                                        </ChatMessageBubble>
                                    </ChatMessageSent>
                                    <ChatMessageReceived class="justify-start items-end mr-auto space-x-2">
                                        <Avatar>
                                            <AvatarImage
                                                attr:alt="The Joker"
                                                attr:src="https://randomuser.me/api/portraits/men/61.jpg"
                                            />
                                        </Avatar>
                                        <ChatMessageBubble class="py-2.5 px-4 shadow-sm bg-muted">
                                            <ChatMessageContent>
                                                "Thank you. Please check your spam folder as well. I can also trigger another reset link if needed."
                                            </ChatMessageContent>
                                            <ChatMessageTime class="mt-1.5 text-muted-foreground/70">
                                                "14:04"
                                            </ChatMessageTime>
                                        </ChatMessageBubble>
                                    </ChatMessageReceived>
                                    <div></div>
                                </ChatMessageList>
                            </div>
                        </div>
                    </div>
                </ChatBody>
                <ChatFooter class="p-4 rounded-b-lg bg-background">
                    <form class="flex items-center space-x-3 w-full">
                        <Input class="flex-1" attr:placeholder="Type your message..." attr:autocomplete="off" />
                        <Button attr:r#type="submit" attr:disabled=true>
                            <Send class="mr-2 w-4 h-4" />
                            "Send"
                        </Button>
                    </form>
                </ChatFooter>
            </ChatCard>
        </section>
    }
}
```