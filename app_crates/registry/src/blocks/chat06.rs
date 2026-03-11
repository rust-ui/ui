/*
 * title: Chat App with Sidebar
 */

use icons::{Bot, Menu, Paperclip, Search, Send, Smile, X};
use leptos::prelude::*;

use crate::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::input::Input;

#[component]
pub fn Chat06() -> impl IntoView {
    view! {
        <div class="flex overflow-hidden my-16 mx-auto w-full max-w-5xl rounded-lg border shadow-lg h-[80vh] max-h-[850px] min-h-[500px] bg-card">
            <div class="flex absolute inset-y-0 left-0 z-20 flex-col w-full border-r transition-transform -translate-x-full md:relative md:w-2/5 md:translate-x-0 bg-background md:min-w-[280px] md:max-w-[400px]">
                <div class="flex justify-between items-center p-3 md:hidden">
                    <h2 class="text-sm font-medium">"Conversations"</h2>
                    <Button variant=ButtonVariant::Ghost size=ButtonSize::Icon class="w-8 h-8">
                        <X class="w-4 h-4" />
                        <span class="hidden">"Close sidebar"</span>
                    </Button>
                </div>
                <div class="p-3">
                    <div class="relative">
                        <Search class="absolute left-2.5 top-1/2 w-4 h-4 -translate-y-1/2 text-muted-foreground" />
                        <Input class="pl-8" attr:placeholder="Search chats..." />
                    </div>
                </div>
                <div
                    dir="ltr"
                    class="overflow-hidden relative flex-1 p-2"
                    style="position: relative; --radix-scroll-area-corner-width: 0px; --radix-scroll-area-corner-height: 0px;"
                >
                    <div
                        data-radix-scroll-area-viewport=""
                        class="w-full h-full rounded-[inherit]"
                        style="overflow: hidden scroll;"
                    >
                        <div style="min-width: 100%; display: table;">
                            <div class="space-y-1">
                                <button class="inline-flex gap-2 justify-start items-center py-2.5 px-3 w-full h-auto text-sm font-medium whitespace-nowrap rounded-md transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-none disabled:opacity-50 disabled:pointer-events-none ring-offset-background [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 bg-secondary text-secondary-foreground hover:bg-secondary/80 focus-visible:ring-ring">
                                    <div class="flex items-center space-x-3 w-full">
                                        <Avatar class="ring-2 ring-offset-2 size-9 ring-primary ring-offset-background">
                                            <AvatarFallback>
                                                <Bot class="w-5 h-5" />
                                            </AvatarFallback>
                                        </Avatar>
                                        <div class="overflow-hidden flex-1">
                                            <p class="text-sm font-medium leading-tight truncate">"Acme Support"</p>
                                            <p class="text-xs truncate text-muted-foreground">
                                                "Okay, let me check your account details."
                                            </p>
                                        </div>
                                        <div class="flex flex-col items-end self-start ml-auto">
                                            <p class="whitespace-nowrap text-[11px] text-muted-foreground/80">
                                                "10m ago"
                                            </p>
                                        </div>
                                    </div>
                                </button>
                                <button class="inline-flex gap-2 justify-start items-center py-2.5 px-3 w-full h-auto text-sm font-medium whitespace-nowrap rounded-md transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-none disabled:opacity-50 disabled:pointer-events-none ring-offset-background [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 hover:bg-accent hover:text-accent-foreground focus-visible:ring-ring">
                                    <div class="flex items-center space-x-3 w-full">
                                        <Avatar class="size-9">
                                            <AvatarImage
                                                attr:alt="Bob Johnson"
                                                attr:src="https://randomuser.me/api/portraits/men/41.jpg"
                                            />
                                        </Avatar>
                                        <div class="overflow-hidden flex-1">
                                            <p class="text-sm font-medium leading-tight truncate">"Bob Johnson"</p>
                                            <p class="text-xs truncate text-muted-foreground">
                                                "Sounds good, see you then!"
                                            </p>
                                        </div>
                                        <div class="flex flex-col items-end self-start ml-auto">
                                            <p class="whitespace-nowrap text-[11px] text-muted-foreground/80">
                                                "1h ago"
                                            </p>
                                            <div class="inline-flex items-center py-0.5 px-1.5 mt-1 h-5 font-semibold rounded-full border border-transparent transition-colors focus:ring-2 focus:ring-offset-2 focus:outline-none bg-primary text-primary-foreground text-[10px] hover:bg-primary/80 focus:ring-ring">
                                                "2"
                                            </div>
                                        </div>
                                    </div>
                                </button>
                                <button class="inline-flex gap-2 justify-start items-center py-2.5 px-3 w-full h-auto text-sm font-medium whitespace-nowrap rounded-md transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-none disabled:opacity-50 disabled:pointer-events-none ring-offset-background [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 hover:bg-accent hover:text-accent-foreground focus-visible:ring-ring">
                                    <div class="flex items-center space-x-3 w-full">
                                        <Avatar class="size-9">
                                            <AvatarImage
                                                attr:alt="Charlie Lee"
                                                attr:src="https://randomuser.me/api/portraits/men/42.jpg"
                                            />
                                        </Avatar>
                                        <div class="overflow-hidden flex-1">
                                            <p class="text-sm font-medium leading-tight truncate">"Charlie Lee"</p>
                                            <p class="text-xs truncate text-muted-foreground">
                                                "Can you resend the invoice?"
                                            </p>
                                        </div>
                                        <div class="flex flex-col items-end self-start ml-auto">
                                            <p class="whitespace-nowrap text-[11px] text-muted-foreground/80">
                                                "3h ago"
                                            </p>
                                        </div>
                                    </div>
                                </button>
                                <button class="inline-flex gap-2 justify-start items-center py-2.5 px-3 w-full h-auto text-sm font-medium whitespace-nowrap rounded-md transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-none disabled:opacity-50 disabled:pointer-events-none ring-offset-background [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 hover:bg-accent hover:text-accent-foreground focus-visible:ring-ring">
                                    <div class="flex items-center space-x-3 w-full">
                                        <Avatar class="size-9">
                                            <AvatarFallback>
                                                <Bot class="w-5 h-5" />
                                            </AvatarFallback>
                                        </Avatar>
                                        <div class="overflow-hidden flex-1">
                                            <p class="text-sm font-medium leading-tight truncate">"Billing Help"</p>
                                            <p class="text-xs truncate text-muted-foreground">
                                                "Your request #459 has been updated."
                                            </p>
                                        </div>
                                        <div class="flex flex-col items-end self-start ml-auto">
                                            <p class="whitespace-nowrap text-[11px] text-muted-foreground/80">
                                                "Yesterday"
                                            </p>
                                            <div class="inline-flex items-center py-0.5 px-1.5 mt-1 h-5 font-semibold rounded-full border border-transparent transition-colors focus:ring-2 focus:ring-offset-2 focus:outline-none bg-primary text-primary-foreground text-[10px] hover:bg-primary/80 focus:ring-ring">
                                                "1"
                                            </div>
                                        </div>
                                    </div>
                                </button>
                                <button class="inline-flex gap-2 justify-start items-center py-2.5 px-3 w-full h-auto text-sm font-medium whitespace-nowrap rounded-md transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-none disabled:opacity-50 disabled:pointer-events-none ring-offset-background [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 hover:bg-accent hover:text-accent-foreground focus-visible:ring-ring">
                                    <div class="flex items-center space-x-3 w-full">
                                        <Avatar class="size-9">
                                            <AvatarImage
                                                attr:alt="Alice Green"
                                                attr:src="https://randomuser.me/api/portraits/women/43.jpg"
                                            />
                                        </Avatar>
                                        <div class="overflow-hidden flex-1">
                                            <p class="text-sm font-medium leading-tight truncate">"Alice Green"</p>
                                            <p class="text-xs truncate text-muted-foreground">
                                                "Thanks for the update!"
                                            </p>
                                        </div>
                                        <div class="flex flex-col items-end self-start ml-auto">
                                            <p class="whitespace-nowrap text-[11px] text-muted-foreground/80">
                                                "Yesterday"
                                            </p>
                                        </div>
                                    </div>
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="flex flex-col flex-1 ml-0">
                <div class="flex items-center p-3 space-x-3 border-b">
                    <Button variant=ButtonVariant::Ghost size=ButtonSize::Icon class="w-8 h-8 md:hidden">
                        <Menu class="w-4 h-4" />
                        <span class="hidden">"Open sidebar"</span>
                    </Button>
                    <Avatar class="size-9">
                        <AvatarFallback>
                            <Bot class="w-5 h-5" />
                        </AvatarFallback>
                    </Avatar>
                    <div>
                        <p class="text-sm font-medium leading-none">"Acme Support"</p>
                        <p class="text-xs text-muted-foreground">"Online Support Agent"</p>
                    </div>
                </div>
                <div
                    dir="ltr"
                    class="overflow-hidden relative flex-1 p-4"
                    style="position: relative; --radix-scroll-area-corner-width: 0px; --radix-scroll-area-corner-height: 0px;"
                >
                    <div
                        data-radix-scroll-area-viewport=""
                        class="w-full h-full rounded-[inherit]"
                        style="overflow: hidden scroll;"
                    >
                        <div style="min-width: 100%; display: table;">
                            <div class="space-y-4">
                                <div class="flex flex-row-reverse items-start ml-auto space-x-2 space-x-reverse max-w-[85%]">
                                    <Avatar>
                                        <AvatarImage
                                            attr:alt="You"
                                            attr:src="https://randomuser.me/api/portraits/lego/5.jpg"
                                        />
                                    </Avatar>
                                    <div class="flex flex-col items-end">
                                        <div class="relative py-2 px-3 text-sm rounded-lg shadow-sm bg-primary text-primary-foreground">
                                            <p class="leading-snug">
                                                "Hi, I'm having trouble logging into my account."
                                            </p>
                                            <p class="mt-1 text-right text-[10px] text-primary-foreground/70">
                                                "11:30 AM"
                                            </p>
                                        </div>
                                    </div>
                                </div>
                                <div class="flex items-start mr-auto space-x-2 max-w-[85%]">
                                    <Avatar>
                                        <AvatarImage
                                            attr:alt="Acme Support"
                                            attr:src="https://randomuser.me/api/portraits/men/50.jpg"
                                        />
                                    </Avatar>
                                    <div class="flex flex-col items-start">
                                        <p class="mb-0.5 text-xs font-medium text-muted-foreground">"Acme Support"</p>
                                        <div class="relative py-2 px-3 text-sm rounded-lg shadow-sm bg-muted">
                                            <p class="leading-snug">
                                                "Hello! Sorry to hear that. Can you please provide your username or email address?"
                                            </p>
                                            <p class="mt-1 text-right text-[10px] text-muted-foreground/70">
                                                "11:31 AM"
                                            </p>
                                        </div>
                                    </div>
                                </div>
                                <div class="flex flex-row-reverse items-start ml-auto space-x-2 space-x-reverse max-w-[85%]">
                                    <Avatar>
                                        <AvatarImage
                                            attr:alt="You"
                                            attr:src="https://randomuser.me/api/portraits/lego/5.jpg"
                                        />
                                    </Avatar>
                                    <div class="flex flex-col items-end">
                                        <div class="relative py-2 px-3 text-sm rounded-lg shadow-sm bg-primary text-primary-foreground">
                                            <p class="leading-snug">"It works! Thank you so much!"</p>
                                            <p class="mt-1 text-right text-[10px] text-primary-foreground/70">
                                                "11:36 AM"
                                            </p>
                                        </div>
                                    </div>
                                </div>
                                <div></div>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="p-3 border-t bg-background">
                    <form class="flex items-center space-x-2 w-full">
                        <div class="hidden sm:block">
                            <Button variant=ButtonVariant::Ghost size=ButtonSize::Icon attr:r#type="button">
                                <Paperclip class="w-5 h-5 text-muted-foreground" />
                                <span class="hidden">"Attach File"</span>
                            </Button>
                        </div>
                        <div class="hidden sm:block">
                            <Button variant=ButtonVariant::Ghost size=ButtonSize::Icon attr:r#type="button">
                                <Smile class="w-5 h-5 text-muted-foreground" />
                                <span class="hidden">"Add Emoji"</span>
                            </Button>
                        </div>
                        <Input class="flex-1" attr:placeholder="Describe your issue..." attr:autocomplete="off" />
                        <Button size=ButtonSize::Sm attr:r#type="submit" attr:disabled=true>
                            <Send class="mr-1.5 w-4 h-4" />
                            <span class="hidden sm:inline">"Send"</span>
                        </Button>
                    </form>
                </div>
            </div>
        </div>
    }
}
