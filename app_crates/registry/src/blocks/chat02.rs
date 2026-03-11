/*
 * title: Team Group Chat
 */

use icons::{Paperclip, Send, Smile, User};
use leptos::prelude::*;

use crate::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::input::Input;

#[component]
pub fn Chat02() -> impl IntoView {
    view! {
        <section class="container py-16 mx-auto">
            <div class="flex flex-col mx-auto w-full max-w-sm rounded-lg border shadow-lg bg-card text-card-foreground h-[75vh] max-h-[800px] min-h-[600px]">
                <div class="flex flex-row justify-between items-center p-3 space-y-1.5 border-b">
                    <div class="flex items-center space-x-3">
                        <Avatar class="border size-9 border-muted">
                            <AvatarFallback>
                                <User class="w-5 h-5" />
                            </AvatarFallback>
                        </Avatar>
                        <div>
                            <p class="text-sm font-medium leading-none">"Project Phoenix Team"</p>
                            <p class="text-xs text-muted-foreground">"4 Members Online"</p>
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
                                <div class="space-y-5">
                                    <div class="flex items-start mr-auto space-x-2 max-w-[85%]">
                                        <Avatar>
                                            <AvatarImage
                                                attr:alt="Alice"
                                                attr:src="https://randomuser.me/api/portraits/women/33.jpg"
                                            />
                                        </Avatar>
                                        <div class="flex flex-col items-start">
                                            <p class="mb-0.5 text-xs font-medium text-muted-foreground">"Alice"</p>
                                            <div class="relative py-2 px-3 text-sm rounded-md border shadow-sm bg-accent">
                                                <p class="leading-snug">
                                                    "Hey team, the staging server is updated. Please test the new login flow."
                                                </p>
                                                <p class="mt-1 text-right text-[10px] text-muted-foreground/70">
                                                    "09:15 AM"
                                                </p>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="flex items-start mr-auto space-x-2 max-w-[85%]">
                                        <Avatar>
                                            <AvatarImage
                                                attr:alt="Bob"
                                                attr:src="https://randomuser.me/api/portraits/men/34.jpg"
                                            />
                                        </Avatar>
                                        <div class="flex flex-col items-start">
                                            <p class="mb-0.5 text-xs font-medium text-muted-foreground">"Bob"</p>
                                            <div class="relative py-2 px-3 text-sm rounded-md border shadow-sm bg-accent">
                                                <p class="leading-snug">"On it! Will report back soon."</p>
                                                <p class="mt-1 text-right text-[10px] text-muted-foreground/70">
                                                    "09:16 AM"
                                                </p>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="flex flex-row-reverse items-start ml-auto space-x-2 space-x-reverse max-w-[85%]">
                                        <Avatar>
                                            <AvatarImage
                                                attr:alt="You"
                                                attr:src="https://randomuser.me/api/portraits/lego/1.jpg"
                                            />
                                        </Avatar>
                                        <div class="flex flex-col items-end">
                                            <div class="relative py-2 px-3 text-sm rounded-md shadow-sm bg-primary text-primary-foreground">
                                                <p class="leading-snug">
                                                    "Looks good on my end. Tested with Chrome and Firefox."
                                                </p>
                                                <p class="mt-1 text-right text-[10px] text-primary-foreground/70">
                                                    "09:20 AM"
                                                </p>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="flex items-start mr-auto space-x-2 max-w-[85%]">
                                        <Avatar>
                                            <AvatarImage
                                                attr:alt="Charlie"
                                                attr:src="https://randomuser.me/api/portraits/men/35.jpg"
                                            />
                                        </Avatar>
                                        <div class="flex flex-col items-start">
                                            <p class="mb-0.5 text-xs font-medium text-muted-foreground">"Charlie"</p>
                                            <div class="relative py-2 px-3 text-sm rounded-md border shadow-sm bg-accent">
                                                <p class="leading-snug">
                                                    "Found a small issue on mobile Safari, the button alignment is slightly off."
                                                </p>
                                                <p class="mt-1 text-right text-[10px] text-muted-foreground/70">
                                                    "09:22 AM"
                                                </p>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="flex flex-row-reverse items-start ml-auto space-x-2 space-x-reverse max-w-[85%]">
                                        <Avatar>
                                            <AvatarImage
                                                attr:alt="You"
                                                attr:src="https://randomuser.me/api/portraits/lego/1.jpg"
                                            />
                                        </Avatar>
                                        <div class="flex flex-col items-end">
                                            <div class="relative py-2 px-3 text-sm rounded-md shadow-sm bg-primary text-primary-foreground">
                                                <p class="leading-snug">
                                                    "Thanks, Charlie. Can you share a screenshot?"
                                                </p>
                                                <p class="mt-1 text-right text-[10px] text-primary-foreground/70">
                                                    "09:23 AM"
                                                </p>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="flex items-start mr-auto space-x-2 max-w-[85%]">
                                        <Avatar>
                                            <AvatarImage
                                                attr:alt="Alice"
                                                attr:src="https://randomuser.me/api/portraits/women/33.jpg"
                                            />
                                        </Avatar>
                                        <div class="flex flex-col items-start">
                                            <p class="mb-0.5 text-xs font-medium text-muted-foreground">"Alice"</p>
                                            <div class="relative py-2 px-3 text-sm rounded-md border shadow-sm bg-accent">
                                                <p class="leading-snug">
                                                    "@Charlie Good catch. Let's create a ticket for that."
                                                </p>
                                                <p class="mt-1 text-right text-[10px] text-muted-foreground/70">
                                                    "09:24 AM"
                                                </p>
                                            </div>
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
                        <Input class="flex-1" attr:placeholder="Message #project-phoenix..." attr:autocomplete="off" />
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
