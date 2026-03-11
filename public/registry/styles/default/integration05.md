---
title: "Integration05"
name: "integration05"
cargo_dependencies: []
registry_dependencies: ["button"]
type: "components:blocks"
path: "blocks/integration05.rs"
---

# Integration05

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add integration05
```

## Component Code

```rust
use icons::{
    Atom, Boxes, CircleDot, Code, Cpu, Database, Globe, Layers, Package, Search, Shield, Sparkles, Square, Star,
    Triangle, Wind, Zap,
};
use leptos::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};

#[component]
pub fn Integration05() -> impl IntoView {
    view! {
        <main role="main">
            <div data-theme="quartz" class="scheme-light bg-background">
                <section class="bg-muted">
                    <div class="py-24 bg-muted/50">
                        <div class="px-6 mx-auto max-w-5xl perspective-dramatic group">
                            <div class="relative justify-between items-center pb-1 mx-auto space-y-6 max-w-2xl from-transparent transition-transform duration-1000 scale-y-90 hover:scale-y-100 rotate-x-6 mask-radial-from-70% mask-radial-[50%_90%] group hover:rotate-x-0">
                                <div class="absolute inset-0 opacity-25 mask-radial-to-55% bg-[radial-gradient(var(--color-foreground)_1px,transparent_1px)] [background-size:16px_16px]"></div>
                                <div>
                                    <div class="overflow-hidden">
                                        <div
                                            class="flex w-max"
                                            style="gap: 56px; flex-direction: row; transform: translateX(-73.98px);"
                                        >
                                            <IconCard>
                                                <Square />
                                            </IconCard>
                                            <IconCard>
                                                <Atom />
                                            </IconCard>
                                            <IconCard>
                                                <Wind />
                                            </IconCard>
                                            <IconCard>
                                                <CircleDot />
                                            </IconCard>
                                            <IconCard>
                                                <Star />
                                            </IconCard>
                                            <IconCard>
                                                <Sparkles />
                                            </IconCard>
                                            <IconCard>
                                                <Zap />
                                            </IconCard>
                                            <IconCard>
                                                <Cpu />
                                            </IconCard>
                                            <IconCard>
                                                <Boxes />
                                            </IconCard>
                                            <IconCard>
                                                <Layers />
                                            </IconCard>
                                            <IconCard>
                                                <Package />
                                            </IconCard>
                                            <IconCard>
                                                <Code />
                                            </IconCard>
                                        </div>
                                    </div>
                                </div>
                                <div>
                                    <div class="overflow-hidden">
                                        <div
                                            class="flex w-max"
                                            style="gap: 56px; flex-direction: row; transform: translateX(-115.903px);"
                                        >
                                            <IconCard>
                                                <Database />
                                            </IconCard>
                                            <IconCard>
                                                <Globe />
                                            </IconCard>
                                            <IconCard>
                                                <Triangle />
                                            </IconCard>
                                            <IconCard>
                                                <Shield />
                                            </IconCard>
                                            <IconCard>
                                                <Star />
                                            </IconCard>
                                            <IconCard>
                                                <Zap />
                                            </IconCard>
                                            <IconCard>
                                                <Cpu />
                                            </IconCard>
                                            <IconCard>
                                                <Boxes />
                                            </IconCard>
                                            <IconCard>
                                                <Layers />
                                            </IconCard>
                                            <IconCard>
                                                <Package />
                                            </IconCard>
                                            <IconCard>
                                                <Code />
                                            </IconCard>
                                            <IconCard>
                                                <Atom />
                                            </IconCard>
                                        </div>
                                    </div>
                                </div>
                                <div>
                                    <div class="overflow-hidden">
                                        <div
                                            class="flex w-max"
                                            style="gap: 56px; flex-direction: row; transform: translateX(-382.935px);"
                                        >
                                            <IconCard>
                                                <Wind />
                                            </IconCard>
                                            <IconCard>
                                                <Sparkles />
                                            </IconCard>
                                            <IconCard>
                                                <Database />
                                            </IconCard>
                                            <IconCard>
                                                <Globe />
                                            </IconCard>
                                            <IconCard>
                                                <Shield />
                                            </IconCard>
                                            <IconCard>
                                                <Star />
                                            </IconCard>
                                            <IconCard>
                                                <Zap />
                                            </IconCard>
                                            <IconCard>
                                                <Cpu />
                                            </IconCard>
                                            <IconCard>
                                                <Boxes />
                                            </IconCard>
                                            <IconCard>
                                                <Layers />
                                            </IconCard>
                                            <IconCard>
                                                <Package />
                                            </IconCard>
                                            <IconCard>
                                                <Code />
                                            </IconCard>
                                        </div>
                                    </div>
                                </div>
                                <div class="flex absolute inset-0 gap-2 justify-center m-auto -translate-y-3.5 size-fit">
                                    <div class="relative z-20 p-1 rounded-2xl border bg-muted">
                                        <div class="flex relative rounded-xl border shadow-xl bg-background shadow-black-950/10 size-24 border-black/25 dark:bg-background dark:border-white/25 dark:shadow-white/10">
                                            <div class="m-auto size-fit *:size-8">
                                                <Search class="text-muted-foreground" />
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            <div class="mx-auto mt-12 max-w-xl text-center">
                                <h2 class="text-3xl font-semibold md:text-4xl text-balance">
                                    Thousands of Icons with your favorite Tools
                                </h2>
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
            </div>
        </main>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn IconCard(children: Children) -> impl IntoView {
    view! {
        <div aria-hidden="true" class="flex relative z-20 rounded-xl border bg-card size-20">
            <div class="m-auto size-fit *:size-8 *:text-muted-foreground">{children()}</div>
        </div>
    }
}
```
