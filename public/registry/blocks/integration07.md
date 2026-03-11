


```rust
use icons::{Circle, LifeBuoy, Sparkles, Square, Star, Triangle, Wind};
use leptos::prelude::*;

use crate::components::ui::badge::{Badge, BadgeVariant};
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn Integration07() -> impl IntoView {
    view! {
        <section class="bg-muted">
            <div class="py-24 bg-muted/50">
                <div class="px-6 mx-auto max-w-5xl">
                    <div class="relative">
                        <div class="inset-0 pb-20 m-auto space-y-5 w-full max-w-xl text-center md:absolute !aspect-auto h-fit">
                            <Badge variant=BadgeVariant::Outline>Consistent style across all icons</Badge>
                            <h2 class="text-2xl font-semibold md:text-4xl text-balance">Thousands of Icons</h2>
                            <p class="text-muted-foreground text-balance">
                                Elevate your designs with our curated icon library. Every icon is crafted with attention to detail, ensuring perfect harmony in your interface.
                            </p>
                            <Button variant=ButtonVariant::Outline size=ButtonSize::Sm href="#">
                                Browse Icons
                            </Button>
                        </div>

                        <div class="grid gap-1 md:grid-rows-6 *:aspect-square max-md:mt-12 md:grid-cols-18">
                            <div aria-hidden="true" class="flex col-start-3 md:row-start-2 max-md:hidden">
                                <div class="mt-auto ml-auto rounded-lg border translate-x-full bg-muted/50 size-1/2 translate-y-[125%]"></div>
                            </div>
                            <div aria-hidden="true" class="flex md:row-start-5 max-md:hidden">
                                <div class="ml-auto rounded-lg border md:translate-x-full bg-muted/50 size-1/2 md:-translate-y-[125%]"></div>
                            </div>
                            <div aria-hidden="true" class="flex md:row-start-3 col-start-16 max-md:hidden">
                                <div class="rounded-lg border md:-translate-x-full bg-muted size-1/2 md:-translate-y-[125%]"></div>
                            </div>
                            <div aria-hidden="true" class="flex md:row-start-5 col-start-18 max-md:hidden">
                                <div class="rounded-lg border md:-translate-x-full bg-muted size-1/2 md:-translate-y-[125%]"></div>
                            </div>
                            <div class="col-start-3">
                                <IconCircle>
                                    <Sparkles />
                                </IconCircle>
                            </div>
                            <div class="col-start-9 md:translate-x-1/2 md:row-start-8">
                                <IconCircle>
                                    <Square />
                                </IconCircle>
                            </div>
                            <div class="md:row-start-3">
                                <IconCircle>
                                    <Star />
                                </IconCircle>
                            </div>
                            <div class="col-start-3 md:row-start-5">
                                <IconCircle>
                                    <Triangle />
                                </IconCircle>
                            </div>
                            <div class="col-start-16">
                                <IconCircle>
                                    <Circle />
                                </IconCircle>
                            </div>
                            <div class="md:row-start-3 col-start-18">
                                <IconCircle>
                                    <LifeBuoy />
                                </IconCircle>
                            </div>
                            <div class="md:row-start-5 col-start-16">
                                <IconCircle>
                                    <Wind />
                                </IconCircle>
                            </div>
                        </div>
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
pub fn IconCircle(children: Children) -> impl IntoView {
    view! {
        <div class="flex relative z-20 m-auto rounded-full border border-transparent ring-1 shadow-md bg-background ring-foreground/10 size-full">
            <div class="m-auto size-fit *:size-5 *:text-muted-foreground">{children()}</div>
        </div>
    }
}
```