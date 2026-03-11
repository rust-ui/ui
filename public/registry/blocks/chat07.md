


```rust

use icons::{Ellipsis, Image, Info, Paperclip, Phone, Plus, Send, Smile, SquarePen, Video};
use leptos::prelude::*;

use crate::components::ui::avatar::{Avatar, AvatarImage};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::input::Input;

#[component]
pub fn Chat07() -> impl IntoView {
    view! {
        <div>
            <div class="flex overflow-hidden my-16 mx-auto w-full max-w-5xl rounded-lg border shadow-xl h-[85vh] max-h-[900px] min-h-[600px] bg-card">
                <div class="flex flex-col w-full border-r bg-muted/20 md:w-[300px]">
                    <div class="flex justify-between items-center px-4 h-16 border-b">
                        <h2 class="text-lg font-semibold">"Chats (4)"</h2>
                        <div class="space-x-1">
                            <Button variant=ButtonVariant::Ghost size=ButtonSize::Icon class="w-8 h-8">
                                <Ellipsis class="w-5 h-5 text-muted-foreground" />
                                <span class="hidden">"More options"</span>
                            </Button>
                            <Button variant=ButtonVariant::Ghost size=ButtonSize::Icon class="w-8 h-8">
                                <SquarePen class="w-5 h-5 text-muted-foreground" />
                                <span class="hidden">"New Chat"</span>
                            </Button>
                        </div>
                    </div>
                    <div
                        dir="ltr"
                        class="overflow-hidden relative flex-1 py-2"
                        style="position: relative; --radix-scroll-area-corner-width: 0px; --radix-scroll-area-corner-height: 0px;"
                    >
                        <div
                            data-radix-scroll-area-viewport=""
                            class="w-full h-full rounded-[inherit]"
                            style="overflow: hidden scroll;"
                        >
                            <div style="min-width: 100%; display: table;">
                                <div class="px-2">
                                    <button class="inline-flex gap-2 justify-start items-center py-2.5 px-3 mb-1 w-full h-auto text-sm font-medium whitespace-nowrap rounded-md transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-none disabled:opacity-50 disabled:pointer-events-none ring-offset-background [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 bg-secondary text-secondary-foreground hover:bg-secondary/80 focus-visible:ring-ring">
                                        <div class="flex items-center space-x-3 w-full">
                                            <Avatar class="ring-2 ring-offset-2 size-10 ring-primary ring-offset-background">
                                                <AvatarImage
                                                    attr:alt="Jane Doe"
                                                    attr:src="https://randomuser.me/api/portraits/women/44.jpg"
                                                />
                                            </Avatar>
                                            <div class="overflow-hidden flex-1">
                                                <p class="text-sm font-medium truncate">"Jane Doe"</p>
                                                <p class="text-xs truncate text-muted-foreground">
                                                    "Active 2 mins ago"
                                                </p>
                                            </div>
                                        </div>
                                    </button>
                                    <button class="inline-flex gap-2 justify-start items-center py-2.5 px-3 mb-1 w-full h-auto text-sm font-medium whitespace-nowrap rounded-md transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-none disabled:opacity-50 disabled:pointer-events-none ring-offset-background [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 hover:bg-accent hover:text-accent-foreground focus-visible:ring-ring">
                                        <div class="flex items-center space-x-3 w-full">
                                            <Avatar class="size-10">
                                                <AvatarImage
                                                    attr:alt="John Doe"
                                                    attr:src="https://randomuser.me/api/portraits/men/45.jpg"
                                                />
                                            </Avatar>
                                            <div class="overflow-hidden flex-1">
                                                <p class="text-sm font-medium truncate">"John Doe"</p>
                                                <p class="text-xs truncate text-muted-foreground">"Offline"</p>
                                            </div>
                                        </div>
                                    </button>
                                    <button class="inline-flex gap-2 justify-start items-center py-2.5 px-3 mb-1 w-full h-auto text-sm font-medium whitespace-nowrap rounded-md transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-none disabled:opacity-50 disabled:pointer-events-none ring-offset-background [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 hover:bg-accent hover:text-accent-foreground focus-visible:ring-ring">
                                        <div class="flex items-center space-x-3 w-full">
                                            <Avatar class="size-10">
                                                <AvatarImage
                                                    attr:alt="Elizabeth Smith"
                                                    attr:src="https://randomuser.me/api/portraits/women/46.jpg"
                                                />
                                            </Avatar>
                                            <div class="overflow-hidden flex-1">
                                                <p class="text-sm font-medium truncate">"Elizabeth Smith"</p>
                                                <p class="text-xs truncate text-primary">"Typing..."</p>
                                            </div>
                                        </div>
                                    </button>
                                    <button class="inline-flex gap-2 justify-start items-center py-2.5 px-3 mb-1 w-full h-auto text-sm font-medium whitespace-nowrap rounded-md transition-colors focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-none disabled:opacity-50 disabled:pointer-events-none ring-offset-background [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 hover:bg-accent hover:text-accent-foreground focus-visible:ring-ring">
                                        <div class="flex items-center space-x-3 w-full">
                                            <Avatar class="size-10">
                                                <AvatarImage
                                                    attr:alt="John Smith"
                                                    attr:src="https://randomuser.me/api/portraits/men/47.jpg"
                                                />
                                            </Avatar>
                                            <div class="overflow-hidden flex-1">
                                                <p class="text-sm font-medium truncate">"John Smith"</p>
                                                <p class="text-xs truncate text-muted-foreground">"Online"</p>
                                            </div>
                                        </div>
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="hidden flex-col flex-1 md:flex bg-background">
                    <div class="flex items-center px-4 h-16 border-b">
                        <div class="flex items-center space-x-3">
                            <Avatar class="ring-2 ring-offset-2 size-9 ring-primary ring-offset-background">
                                <AvatarImage
                                    attr:alt="Jane Doe"
                                    attr:src="https://randomuser.me/api/portraits/women/44.jpg"
                                />
                            </Avatar>
                            <div>
                                <p class="text-sm font-medium leading-tight">"Jane Doe"</p>
                                <p class="text-xs leading-tight text-muted-foreground">"Active 2 mins ago"</p>
                            </div>
                        </div>
                        <div class="flex items-center ml-auto space-x-1">
                            <Button variant=ButtonVariant::Ghost size=ButtonSize::Icon>
                                <Video class="w-5 h-5 text-muted-foreground" />
                                <span class="hidden">"Start video call"</span>
                            </Button>
                            <Button variant=ButtonVariant::Ghost size=ButtonSize::Icon>
                                <Phone class="w-5 h-5 text-muted-foreground" />
                                <span class="hidden">"Start audio call"</span>
                            </Button>
                            <Button variant=ButtonVariant::Ghost size=ButtonSize::Icon>
                                <Info class="w-5 h-5 text-muted-foreground" />
                                <span class="hidden">"View chat info"</span>
                            </Button>
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
                                <div class="space-y-1">
                                    <div class="flex justify-start items-end mt-3">
                                        <div class="flex items-end space-x-2 max-w-[75%]">
                                            <div class="w-8 shrink-0">
                                                <Avatar>
                                                    <AvatarImage
                                                        attr:alt="Jane Doe"
                                                        attr:src="https://randomuser.me/api/portraits/women/44.jpg"
                                                    />
                                                </Avatar>
                                            </div>
                                            <div class="py-2 px-3 text-sm rounded-md shadow-sm bg-muted">
                                                <p class="leading-snug">"I am good, you?"</p>
                                                <p class="mt-1 text-right text-[10px] text-muted-foreground/70">
                                                    "10:03 AM"
                                                </p>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="flex justify-end items-end mt-3">
                                        <div class="flex flex-row-reverse items-end space-x-2 space-x-reverse max-w-[75%]">
                                            <div class="w-8 shrink-0">
                                                <Avatar>
                                                    <AvatarImage
                                                        attr:alt="You"
                                                        attr:src="https://randomuser.me/api/portraits/lego/6.jpg"
                                                    />
                                                </Avatar>
                                            </div>
                                            <div class="py-2 px-3 text-sm rounded-md shadow-sm bg-primary text-primary-foreground">
                                                <p class="leading-snug">"I am good too!"</p>
                                                <p class="mt-1 text-right text-[10px] text-primary-foreground/70">
                                                    "10:04 AM"
                                                </p>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="flex justify-end items-end mt-1">
                                        <div class="flex flex-row-reverse items-end space-x-2 space-x-reverse max-w-[75%]">
                                            <div class="w-8 shrink-0"></div>
                                            <div class="py-2 px-3 text-sm rounded-md shadow-sm bg-primary text-primary-foreground">
                                                <p class="leading-snug">"That is good to hear!"</p>
                                                <p class="mt-1 text-right text-[10px] text-primary-foreground/70">
                                                    "10:05 AM"
                                                </p>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="flex justify-start items-end mt-3">
                                        <div class="flex items-end space-x-2 max-w-[75%]">
                                            <div class="w-8 shrink-0">
                                                <Avatar>
                                                    <AvatarImage
                                                        attr:alt="Jane Doe"
                                                        attr:src="https://randomuser.me/api/portraits/women/44.jpg"
                                                    />
                                                </Avatar>
                                            </div>
                                            <div class="py-2 px-3 text-sm rounded-md shadow-sm bg-muted">
                                                <p class="leading-snug">"How has your day been so far?"</p>
                                                <p class="mt-1 text-right text-[10px] text-muted-foreground/70">
                                                    "10:06 AM"
                                                </p>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="flex justify-end items-end mt-3">
                                        <div class="flex flex-row-reverse items-end space-x-2 space-x-reverse max-w-[75%]">
                                            <div class="w-8 shrink-0">
                                                <Avatar>
                                                    <AvatarImage
                                                        attr:alt="You"
                                                        attr:src="https://randomuser.me/api/portraits/lego/6.jpg"
                                                    />
                                                </Avatar>
                                            </div>
                                            <div class="py-2 px-3 text-sm rounded-md shadow-sm bg-primary text-primary-foreground">
                                                <p class="leading-snug">
                                                    "It has been good. I went for a run this morning and then had a nice breakfast. How about you?"
                                                </p>
                                                <p class="mt-1 text-right text-[10px] text-primary-foreground/70">
                                                    "10:10 AM"
                                                </p>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="flex justify-start items-end mt-3">
                                        <div class="flex items-end space-x-2 max-w-[75%]">
                                            <div class="w-8 shrink-0">
                                                <Avatar>
                                                    <AvatarImage
                                                        attr:alt="Jane Doe"
                                                        attr:src="https://randomuser.me/api/portraits/women/44.jpg"
                                                    />
                                                </Avatar>
                                            </div>
                                            <div class="py-2 px-3 text-sm rounded-md shadow-sm bg-muted">
                                                <p class="leading-snug">"Awesome! I am just chilling outside."</p>
                                                <p class="mt-1 text-right text-[10px] text-muted-foreground/70">
                                                    "2:51 PM"
                                                </p>
                                            </div>
                                        </div>
                                    </div>
                                    <div></div>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="p-3 border-t">
                        <form class="flex items-center space-x-2 w-full">
                            <Button variant=ButtonVariant::Ghost size=ButtonSize::Icon>
                                <Plus class="w-5 h-5 text-muted-foreground" />
                                <span class="hidden">"Attach file or media"</span>
                            </Button>
                            <Button variant=ButtonVariant::Ghost size=ButtonSize::Icon>
                                <Image class="w-5 h-5 text-muted-foreground" />
                                <span class="hidden">"Attach image"</span>
                            </Button>
                            <Button variant=ButtonVariant::Ghost size=ButtonSize::Icon>
                                <Paperclip class="w-5 h-5 text-muted-foreground" />
                                <span class="hidden">"Attach file"</span>
                            </Button>
                            <Input
                                class="flex-1 rounded-full"
                                attr:placeholder="Type a message..."
                                attr:autocomplete="off"
                            />
                            <Button variant=ButtonVariant::Ghost size=ButtonSize::Icon>
                                <Smile class="w-5 h-5 text-muted-foreground" />
                                <span class="hidden">"Add emoji"</span>
                            </Button>
                            <Button size=ButtonSize::Icon class="w-8 h-8" attr:r#type="submit" attr:disabled=true>
                                <Send class="w-4 h-4" />
                                <span class="hidden">"Send"</span>
                            </Button>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
```