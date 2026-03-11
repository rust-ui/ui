use icons::{Brush, CircleDot, Compass, LifeBuoy, Search, Sparkles, Square};
use leptos::prelude::*;

use crate::ui::button::{Button, ButtonVariant};

/*
 * title: Icon Library with Centered Grid
*/

#[component]
pub fn Integration02() -> impl IntoView {
    view! {
        <section>
            <div class="py-24 md:py-32 bg-muted dark:bg-background">
                <div class="px-6 mx-auto max-w-5xl">
                    <div class="relative mx-auto w-fit dark:bg-muted/50">
                        <div
                            role="presentation"
                            class="absolute inset-0 z-10 from-transparent bg-radial to-muted to-75% dark:to-background"
                        ></div>
                        <div class="flex gap-2 justify-center mx-auto mb-2 w-fit">
                            <IconWrapper>
                                <Sparkles />
                            </IconWrapper>
                            <IconWrapper>
                                <Square />
                            </IconWrapper>
                        </div>

                        <div class="flex gap-2 justify-center my-2 mx-auto w-fit">
                            <IconWrapper>
                                <Brush />
                            </IconWrapper>

                            <IconWrapperMain>
                                <Search />
                            </IconWrapperMain>

                            <IconWrapper>
                                <CircleDot />
                            </IconWrapper>
                        </div>

                        <div class="flex gap-2 justify-center mx-auto w-fit">
                            <IconWrapper>
                                <LifeBuoy />
                            </IconWrapper>
                            <IconWrapper>
                                <Compass />
                            </IconWrapper>
                        </div>
                    </div>

                    <div class="mx-auto mt-6 space-y-6 max-w-lg text-center">
                        <h2 class="text-3xl font-semibold md:text-4xl text-balance">Thousands of Icons</h2>
                        <p class="mt-3 text-center sm:text-center lg:mt-3 [word-spacing:0.02em] font-book text-muted-foreground">
                            "Elevate your designs with our curated icon library. Every icon is crafted with attention to detail, ensuring perfect
                            harmony in your interface."
                        </p>
                        <Button variant=ButtonVariant::Outline href="#">
                            Get Started
                        </Button>
                    </div>
                </div>
            </div>
        </section>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn IconWrapper(children: Children) -> impl IntoView {
    view! {
        <div data-name="IconWrapper" class="flex relative rounded-xl dark:bg-transparent bg-background size-20">
            <div role="presentation" class="absolute inset-0 rounded-xl border border-black/20 dark:border-white/25" />
            <div class="relative z-20 m-auto size-fit *:size-8 *:text-muted-foreground">{children()}</div>
        </div>
    }
}

#[component]
pub fn IconWrapperMain(children: Children) -> impl IntoView {
    view! {
        <div class="flex relative rounded-xl bg-background size-20 dark:bg-white/10">
            <div
                role="presentation"
                class="absolute inset-0 rounded-xl border shadow-xl shadow-black-950/10 border-black/25 dark:border-white/25"
            />
            <div class="relative z-20 m-auto size-fit *:size-8 *:text-muted-foreground">{children()}</div>
        </div>
    }
}
