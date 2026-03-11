---
title: "Chat04"
name: "chat04"
cargo_dependencies: []
registry_dependencies: ["avatar", "button", "input"]
type: "components:blocks"
path: "blocks/chat04.rs"
---

# Chat04

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add chat04
```

## Component Code

```rust

use icons::{Download, Paperclip, Send, Smile};
use leptos::prelude::*;

use crate::components::ui::avatar::{Avatar, AvatarImage};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::components::ui::input::Input;

#[component]
pub fn Chat04() -> impl IntoView {
    view! {
        <section class="container py-16 mx-auto">
            <div class="flex flex-col mx-auto w-full max-w-sm rounded-lg border shadow-lg bg-card text-card-foreground h-[75vh] max-h-[800px] min-h-[600px]">
                <div class="flex flex-row items-center p-3 space-y-1.5 border-b">
                    <div class="flex items-center space-x-3">
                        <Avatar class="ring-2 ring-offset-2 size-9 ring-primary ring-offset-background">
                            <AvatarImage
                                attr:alt="Designer"
                                attr:src="https://randomuser.me/api/portraits/men/39.jpg"
                            />
                        </Avatar>
                        <div>
                            <p class="text-sm font-medium leading-none">"Designer"</p>
                            <p class="text-xs text-muted-foreground">"Discussing Mockups"</p>
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
                                    <div class="relative py-3">
                                        <div
                                            data-orientation="horizontal"
                                            role="none"
                                            class="absolute inset-x-0 top-1/2 w-full -translate-y-1/2 shrink-0 bg-border h-[1px]"
                                        ></div>
                                        <div class="flex relative justify-center">
                                            <span class="px-3 text-xs font-medium bg-card text-muted-foreground">
                                                "November 15, 2023"
                                            </span>
                                        </div>
                                    </div>
                                    <div class="flex justify-start items-end mr-auto space-x-2">
                                        <Avatar>
                                            <AvatarImage
                                                attr:alt="Designer"
                                                attr:src="https://randomuser.me/api/portraits/men/39.jpg"
                                            />
                                        </Avatar>
                                        <div class="py-2 px-3 text-sm rounded-lg max-w-[75%] bg-muted">
                                            <p class="leading-snug">
                                                "Here's the first draft of the landing page mockup."
                                            </p>
                                            <p class="mt-1 text-right text-[10px] text-muted-foreground/70">
                                                "9:30 AM"
                                            </p>
                                        </div>
                                    </div>
                                    <div class="flex justify-start items-end mr-auto space-x-2">
                                        <Avatar class="opacity-0">
                                            <AvatarImage
                                                attr:alt="Designer"
                                                attr:src="https://randomuser.me/api/portraits/men/39.jpg"
                                            />
                                        </Avatar>
                                        <div class="w-8 shrink-0"></div>
                                        <div class="overflow-hidden p-1 text-sm rounded-lg border max-w-[75%] bg-muted">
                                            <div class="relative group">
                                                <img
                                                    src="https://imagedelivery.net/Kpcbofvpelk1jdjXmWIr5w/15656e6c-1315-435d-fa59-ec0ce2ac0700/public?text=Landing+Page+Draft"
                                                    alt="Chat image"
                                                    class="object-cover max-w-xs max-h-60 rounded"
                                                />
                                                <div class="flex absolute inset-0 justify-center items-center opacity-0 transition-opacity group-hover:opacity-100 bg-black/30">
                                                    <Button
                                                        variant=ButtonVariant::Ghost
                                                        size=ButtonSize::Icon
                                                        class="text-white hover:text-accent-foreground hover:bg-black/50"
                                                    >
                                                        <Download class="w-5 h-5" />
                                                        <span class="hidden">"Download Image"</span>
                                                    </Button>
                                                </div>
                                                <p class="absolute bottom-1 right-1.5 py-0.5 px-1 text-white rounded bg-black/50 text-[9px]">
                                                    "9:31 AM"
                                                </p>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="flex justify-end items-end ml-auto space-x-2">
                                        <div class="py-2 px-3 text-sm rounded-lg max-w-[75%] bg-primary text-primary-foreground">
                                            <p class="leading-snug">"Looks great! I like the color scheme."</p>
                                            <p class="mt-1 text-right text-[10px] text-primary-foreground/70">
                                                "9:35 AM"
                                            </p>
                                        </div>
                                    </div>
                                    <div class="flex justify-end items-end ml-auto space-x-2">
                                        <div class="py-2 px-3 text-sm rounded-lg max-w-[75%] bg-primary text-primary-foreground">
                                            <p class="leading-snug">
                                                "Could we maybe try a version with the CTA button higher up?"
                                            </p>
                                            <p class="mt-1 text-right text-[10px] text-primary-foreground/70">
                                                "9:36 AM"
                                            </p>
                                        </div>
                                    </div>
                                    <div class="relative py-3">
                                        <div
                                            data-orientation="horizontal"
                                            role="none"
                                            class="absolute inset-x-0 top-1/2 w-full -translate-y-1/2 shrink-0 bg-border h-[1px]"
                                        ></div>
                                        <div class="flex relative justify-center">
                                            <span class="px-3 text-xs font-medium bg-card text-muted-foreground">
                                                "November 16, 2023"
                                            </span>
                                        </div>
                                    </div>
                                    <div class="flex justify-start items-end mr-auto space-x-2">
                                        <Avatar>
                                            <AvatarImage
                                                attr:alt="Designer"
                                                attr:src="https://randomuser.me/api/portraits/men/39.jpg"
                                            />
                                        </Avatar>
                                        <div class="py-2 px-3 text-sm rounded-lg max-w-[75%] bg-muted">
                                            <p class="leading-snug">"Sure, let me adjust that. How about this?"</p>
                                            <p class="mt-1 text-right text-[10px] text-muted-foreground/70">
                                                "2:10 PM"
                                            </p>
                                        </div>
                                    </div>
                                    <div class="flex justify-start items-end mr-auto space-x-2">
                                        <Avatar class="opacity-0">
                                            <AvatarImage
                                                attr:alt="Designer"
                                                attr:src="https://randomuser.me/api/portraits/men/39.jpg"
                                            />
                                        </Avatar>
                                        <div class="w-8 shrink-0"></div>
                                        <div class="overflow-hidden p-1 text-sm rounded-lg border max-w-[75%] bg-muted">
                                            <div class="relative group">
                                                <img
                                                    src="https://imagedelivery.net/Kpcbofvpelk1jdjXmWIr5w/15656e6c-1315-435d-fa59-ec0ce2ac0700/public?text=Updated+Mockup"
                                                    alt="Chat image"
                                                    class="object-cover max-w-xs max-h-60 rounded"
                                                />
                                                <div class="flex absolute inset-0 justify-center items-center opacity-0 transition-opacity group-hover:opacity-100 bg-black/30">
                                                    <Button
                                                        variant=ButtonVariant::Ghost
                                                        size=ButtonSize::Icon
                                                        class="text-white hover:text-accent-foreground hover:bg-black/50"
                                                    >
                                                        <Download class="w-5 h-5" />
                                                        <span class="hidden">"Download Image"</span>
                                                    </Button>
                                                </div>
                                                <p class="absolute bottom-1 right-1.5 py-0.5 px-1 text-white rounded bg-black/50 text-[9px]">
                                                    "2:11 PM"
                                                </p>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="flex justify-end items-end ml-auto space-x-2">
                                        <div class="py-2 px-3 text-sm rounded-lg max-w-[75%] bg-primary text-primary-foreground">
                                            <p class="leading-snug">"Perfect! That's exactly what I was thinking."</p>
                                            <p class="mt-1 text-right text-[10px] text-primary-foreground/70">
                                                "2:15 PM"
                                            </p>
                                        </div>
                                    </div>
                                    <div></div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="flex items-center p-3 rounded-b-lg border-t bg-background">
                    <form class="flex items-center space-x-2 w-full">
                        <Button variant=ButtonVariant::Ghost size=ButtonSize::Icon attr:r#type="button">
                            <Paperclip class="w-5 h-5 text-muted-foreground" />
                            <span class="hidden">"Attach File"</span>
                        </Button>
                        <Button variant=ButtonVariant::Ghost size=ButtonSize::Icon attr:r#type="button">
                            <Smile class="w-5 h-5 text-muted-foreground" />
                            <span class="hidden">"Add Emoji"</span>
                        </Button>
                        <Input
                            class="flex-1"
                            attr:placeholder="Type a message or drop a file..."
                            attr:autocomplete="off"
                        />
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
