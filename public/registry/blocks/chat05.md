


```rust

use icons::{Paperclip, Send, Smile};
use leptos::prelude::*;

use crate::components::ui::avatar::{Avatar, AvatarImage};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::input::Input;

#[component]
pub fn Chat05() -> impl IntoView {
    view! {
        <section class="container py-16 mx-auto">
            <div class="flex flex-col mx-auto w-full max-w-3xl h-full rounded-lg border shadow-sm bg-card text-card-foreground">
                <div class="flex flex-row items-center p-3 space-y-1.5 border-b">
                    <div class="flex items-center space-x-3">
                        <Avatar class="ring-2 ring-offset-2 size-9 ring-primary ring-offset-background">
                            <AvatarImage attr:alt="Alice" attr:src="https://randomuser.me/api/portraits/women/40.jpg" />
                        </Avatar>
                        <div>
                            <p class="text-sm font-medium leading-none">"Alice"</p>
                            <p class="text-xs text-muted-foreground">"Online"</p>
                        </div>
                    </div>
                </div>
                <div class="overflow-hidden flex-1 p-0">
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
                                <div class="space-y-4">
                                    <div class="flex justify-start items-end mr-auto space-x-2 max-w-[85%]">
                                        <Avatar>
                                            <AvatarImage
                                                attr:alt="Alice"
                                                attr:src="https://randomuser.me/api/portraits/women/40.jpg"
                                            />
                                        </Avatar>
                                        <div class="relative py-2 px-3 text-sm rounded-lg bg-muted">
                                            <p class="leading-snug">"Hey Bob, ready for the stand-up?"</p>
                                            <p class="mt-1 text-xs text-right text-muted-foreground/70">"10:30 AM"</p>
                                        </div>
                                    </div>
                                    <div class="flex justify-end items-end ml-auto space-x-2 max-w-[85%]">
                                        <div class="relative py-2 px-3 text-sm rounded-lg bg-primary text-primary-foreground">
                                            <p class="leading-snug">
                                                "Morning Alice! Yep, just grabbing my coffee. Almost there."
                                            </p>
                                            <p class="mt-1 text-xs text-right text-primary-foreground/70">"10:31 AM"</p>
                                        </div>
                                    </div>
                                    <div class="flex justify-start items-end mr-auto space-x-2 max-w-[85%]">
                                        <Avatar>
                                            <AvatarImage
                                                attr:alt="Alice"
                                                attr:src="https://randomuser.me/api/portraits/women/40.jpg"
                                            />
                                        </Avatar>
                                        <div class="relative py-2 px-3 text-sm rounded-lg bg-muted">
                                            <p class="leading-snug">
                                                "Sounds good. Did you manage to fix that bug in the auth flow?"
                                            </p>
                                            <p class="mt-1 text-xs text-right text-muted-foreground/70">"10:32 AM"</p>
                                        </div>
                                    </div>
                                    <div class="flex justify-end items-end ml-auto space-x-2 max-w-[85%]">
                                        <div class="relative py-2 px-3 text-sm rounded-lg bg-primary text-primary-foreground">
                                            <p class="leading-snug">
                                                "Yes! It was a silly typo in an environment variable. Pushed the fix last night."
                                            </p>
                                            <p class="mt-1 text-xs text-right text-primary-foreground/70">"10:33 AM"</p>
                                        </div>
                                    </div>
                                    <div class="flex justify-start items-end mr-auto space-x-2 max-w-[85%]">
                                        <Avatar>
                                            <AvatarImage
                                                attr:alt="Alice"
                                                attr:src="https://randomuser.me/api/portraits/women/40.jpg"
                                            />
                                        </Avatar>
                                        <div class="relative py-2 px-3 text-sm rounded-lg bg-muted">
                                            <p class="leading-snug">
                                                "Awesome! That was blocking the QA team. Great job!"
                                            </p>
                                            <p class="mt-1 text-xs text-right text-muted-foreground/70">"10:34 AM"</p>
                                        </div>
                                    </div>
                                    <div class="flex justify-start items-end mr-auto space-x-2 max-w-[85%]">
                                        <Avatar>
                                            <AvatarImage
                                                attr:alt="Alice"
                                                attr:src="https://randomuser.me/api/portraits/women/40.jpg"
                                            />
                                        </Avatar>
                                        <div class="relative py-2 px-3 text-sm rounded-lg bg-muted">
                                            <p class="leading-snug">"See you in the meeting room shortly."</p>
                                            <p class="mt-1 text-xs text-right text-muted-foreground/70">"10:34 AM"</p>
                                        </div>
                                    </div>
                                    <div></div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="flex items-center p-3 border-t">
                    <form class="flex items-center space-x-2 w-full">
                        <Button variant=ButtonVariant::Ghost size=ButtonSize::Icon attr:r#type="button">
                            <Paperclip class="w-5 h-5 text-muted-foreground" />
                            <span class="hidden">"Attach File"</span>
                        </Button>
                        <Button variant=ButtonVariant::Ghost size=ButtonSize::Icon attr:r#type="button">
                            <Smile class="w-5 h-5 text-muted-foreground" />
                            <span class="hidden">"Add Emoji"</span>
                        </Button>
                        <Input class="flex-1" attr:placeholder="Type a message..." attr:autocomplete="off" />
                        <Button size=ButtonSize::Sm attr:r#type="submit" attr:disabled=true>
                            <Send class="mr-1.5 w-4 h-4" />
                            "Send"
                        </Button>
                    </form>
                </div>
            </div>
        </section>
    }
}
```