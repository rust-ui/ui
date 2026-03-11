


```rust
use icons::{CircleDot, LifeBuoy, Search, Sparkles, Square, Star, Wind};
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};

#[component]
pub fn Integration03() -> impl IntoView {
    view! {
        <section>
            <div class="py-24 md:py-32 bg-muted dark:bg-background">
                <div class="px-6 mx-auto max-w-5xl">
                    <div class="flex relative justify-between items-center mx-auto max-w-sm">
                        <div class="space-y-6">
                            <div class="relative">
                                <IconCard>
                                    <Sparkles />
                                </IconCard>
                                <div class="absolute top-1/2 left-full z-10 h-px origin-left bg-linear-to-r to-muted-foreground/25 w-[130px] rotate-[25deg]"></div>
                            </div>
                            <div class="relative">
                                <IconCard>
                                    <Square />
                                </IconCard>
                                <div class="absolute top-1/2 left-full z-10 h-px origin-left bg-linear-to-r to-muted-foreground/25 w-[120px]"></div>
                            </div>
                            <div class="relative">
                                <IconCard>
                                    <Wind />
                                </IconCard>
                                <div class="absolute top-1/2 left-full z-10 h-px origin-left bg-linear-to-r to-muted-foreground/25 w-[130px] rotate-[-25deg]"></div>
                            </div>
                        </div>
                        <div class="flex gap-2 justify-center my-2 mx-auto w-fit">
                            <div class="relative z-20 p-1 rounded-2xl border bg-muted">
                                <div class="flex relative rounded-xl border shadow-xl bg-background shadow-black-950/10 size-16 border-black/25 dark:bg-background dark:border-white/25 dark:shadow-white/10">
                                    <div class="relative z-20 m-auto size-fit *:size-8">
                                        <Search class="text-muted-foreground" />
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div
                            role="presentation"
                            class="absolute inset-1/3 opacity-50 bg-[radial-gradient(var(--dots-color)_1px,transparent_1px)] [--dots-color:black] [background-size:16px_16px] [mask-image:radial-gradient(ellipse_50%_50%_at_50%_50%,#000_70%,transparent_100%)] dark:[--dots-color:white]"
                        ></div>
                        <div class="space-y-6">
                            <div class="relative">
                                <IconCard>
                                    <CircleDot />
                                </IconCard>
                                <div class="absolute top-1/2 right-full z-10 h-px origin-right to-muted-foreground/25 bg-linear-to-l w-[130px] rotate-[-25deg]"></div>
                            </div>
                            <div class="relative">
                                <IconCard>
                                    <LifeBuoy />
                                </IconCard>
                                <div class="absolute top-1/2 right-full z-10 h-px origin-right to-muted-foreground/25 bg-linear-to-l w-[120px]"></div>
                            </div>
                            <div class="relative">
                                <IconCard>
                                    <Star />
                                </IconCard>
                                <div class="absolute top-1/2 right-full z-10 h-px origin-right to-muted-foreground/25 bg-linear-to-l w-[130px] rotate-[25deg]"></div>
                            </div>
                        </div>
                    </div>
                    <div class="mx-auto mt-6 space-y-6 max-w-lg text-center">
                        <h2 class="text-3xl font-semibold md:text-4xl text-balance">Thousands of Icons</h2>
                        <p class="mt-3 text-center sm:text-center lg:mt-3 [word-spacing:0.02em] font-book text-muted-foreground">
                            "Elevate your designs with our curated icon library. Every icon is crafted with attention to detail, ensuring perfect harmony in your interface."
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
fn IconCard(children: Children) -> impl IntoView {
    view! {
        <div class="flex relative rounded-xl border dark:bg-transparent bg-background size-12">
            <div class="relative z-20 m-auto size-fit *:size-6 *:text-muted-foreground">{children()}</div>
        </div>
    }
}
```