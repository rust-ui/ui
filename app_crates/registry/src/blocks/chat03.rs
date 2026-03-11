/*
 * title: AI Chatbot Interface
 */

use icons::{ArrowUp, Bot, ChevronLeft};
use leptos::prelude::*;

use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::input::Input;

#[component]
pub fn Chat03() -> impl IntoView {
    view! {
        <section class="container flex justify-center py-8 mx-auto">
            <div class="flex overflow-hidden flex-col w-full max-w-sm rounded-lg border shadow-lg h-[80vh] min-h-[600px] bg-background">
                <header class="flex items-center px-4 h-14 border-b">
                    <Button variant=ButtonVariant::Ghost size=ButtonSize::Icon class="mr-2">
                        <ChevronLeft class="w-5 h-5" />
                        <span class="hidden">"Back"</span>
                    </Button>
                    <h2 class="flex-1 text-sm font-medium text-center">"chatbot"</h2>
                    <div class="w-8"></div>
                </header>
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
                            <div class="flex flex-col items-center mb-8 text-center">
                                <div class="p-3 mb-4 rounded-lg bg-muted">
                                    <Bot class="w-10 h-10 text-muted-foreground" />
                                </div>
                                <p class="text-lg font-semibold">"AI Agent answers instantly"</p>
                                <p class="text-sm text-muted-foreground">"Ask for the team if needed"</p>
                            </div>
                            <div class="space-y-4">
                                <div class="flex gap-2 justify-start items-start">
                                    <div class="flex justify-center items-center mt-1 w-6 h-6 rounded-sm shrink-0 bg-muted text-muted-foreground">
                                        <Bot class="w-4 h-4" />
                                    </div>
                                    <div class="flex flex-col items-start">
                                        <div class="py-2 px-3 max-w-xs text-sm rounded-lg sm:max-w-sm md:max-w-md bg-muted text-foreground">
                                            <p class="leading-normal">
                                                <span role="img" aria-label="wave" class="mr-1">
                                                    "👋"
                                                </span>
                                                " Hey there! I'm an AI Chatbot."
                                                <br />
                                                "Feel free to ask me anything!"
                                            </p>
                                        </div>
                                        <p class="mt-1 text-xs text-muted-foreground">"Bot · Just now."</p>
                                    </div>
                                </div>
                                <div></div>
                            </div>
                        </div>
                    </div>
                </div>
                <footer class="p-3 border-t bg-card">
                    <form class="flex items-center space-x-2 w-full">
                        <Input
                            class="flex-1 rounded-full"
                            attr:placeholder="Ask a question..."
                            attr:autocomplete="off"
                        />
                        <Button
                            class="rounded-full shrink-0 bg-muted-foreground text-background hover:bg-muted-foreground/90 disabled:bg-muted disabled:text-muted-foreground"
                            size=ButtonSize::Icon
                            attr:r#type="submit"
                            attr:disabled=true
                            attr:title="Send"
                        >
                            <ArrowUp class="w-5 h-5" />
                            <span class="hidden">"Send"</span>
                        </Button>
                    </form>
                </footer>
            </div>
        </section>
    }
}
