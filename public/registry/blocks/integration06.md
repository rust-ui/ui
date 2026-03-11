


```rust
use icons::{
    Atom, Boxes, CircleDot, Code, Cpu, Database, Globe, Heart, Layers, Package, Rocket, Search, Shield, Sparkles,
    Square, Star, Wind, Zap,
};
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};

#[component]
pub fn Integration06() -> impl IntoView {
    view! {
        <section>
            <div class="py-24 md:py-32 bg-muted dark:bg-background">
                <div class="px-6 mx-auto max-w-5xl">
                    <div class="relative justify-between items-center mx-auto space-y-6 sm:max-w-md bg-muted/25 group max-w-[22rem] [mask-image:radial-gradient(ellipse_50%_50%_at_50%_50%,#000_70%,transparent_100%)]">
                        <div
                            role="presentation"
                            class="absolute inset-0 opacity-50 -z-10 bg-[linear-gradient(to_right,var(--color-border)_1px,transparent_1px),linear-gradient(to_bottom,#4f4f4f2e_1px,transparent_1px)] bg-[size:32px_32px]"
                        ></div>
                        <div>
                            <div class="overflow-hidden">
                                <div
                                    class="flex w-max"
                                    style="gap: 24px; flex-direction: row; transform: translateX(-119.64px);"
                                >
                                    <IconBubble>
                                        <CircleDot class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Atom class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Sparkles class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Star class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Square class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Wind class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Zap class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Cpu class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Boxes class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Layers class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Package class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Code class="text-muted-foreground" />
                                    </IconBubble>
                                </div>
                            </div>
                        </div>
                        <div>
                            <div class="overflow-hidden">
                                <div
                                    class="flex w-max"
                                    style="gap: 24px; flex-direction: row; transform: translateX(-311.08px);"
                                >
                                    <IconBubble>
                                        <Sparkles class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Square class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Atom class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Wind class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <CircleDot class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Database class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Globe class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Shield class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Rocket class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Heart class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Star class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Zap class="text-muted-foreground" />
                                    </IconBubble>
                                </div>
                            </div>
                        </div>
                        <div>
                            <div class="overflow-hidden">
                                <div
                                    class="flex w-max"
                                    style="gap: 24px; flex-direction: row; transform: translateX(-119.64px);"
                                >
                                    <IconBubble>
                                        <Square class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Wind class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Sparkles class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <CircleDot class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Atom class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Cpu class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Boxes class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Layers class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Package class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Code class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Database class="text-muted-foreground" />
                                    </IconBubble>
                                    <IconBubble>
                                        <Globe class="text-muted-foreground" />
                                    </IconBubble>
                                </div>
                            </div>
                        </div>
                        <div class="flex absolute inset-0 gap-2 justify-center m-auto size-fit">
                            <div class="flex relative z-20 rounded-full border shadow-xl shadow-black-950/10 size-16 bg-white/25 backdrop-blur-md backdrop-grayscale dark:border-white/10 dark:shadow-white/15">
                                <div class="m-auto size-fit *:size-8">
                                    <Search class="text-muted-foreground" />
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="mx-auto mt-12 space-y-6 max-w-lg text-center">
                        <h2 class="text-3xl font-semibold md:text-4xl text-balance">Thousands of Icons</h2>
                        <p class="mt-3 text-center sm:text-center lg:mt-3 [word-spacing:0.02em] font-book text-muted-foreground">
                            Elevate your designs with our curated icon library. Every icon is crafted with attention to detail, ensuring perfect harmony in your interface.
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
pub fn IconBubble(children: Children) -> impl IntoView {
    view! {
        <div class="flex relative z-20 rounded-full border bg-background size-12">
            <div class="m-auto size-fit *:size-5">{children()}</div>
        </div>
    }
}
```