use icons::{Boxes, Circle, Component, Cuboid, Grid3X3, Network, Package, Square};
use leptos::prelude::*;
use leptos_ui::clx;

use crate::ui::image::Image;

/*
 * title: Icon Library Showcase
*/

#[component]
pub fn Integration01() -> impl IntoView {
    clx! {IconWrapper, div, "flex justify-center items-center bg-background rounded-lg border shadow-lg size-12 [&_svg:not([class*='size-'])]:size-6"}

    view! {
        <section class="relative pt-24 w-full lg:pt-36 bg-background">
            <div>
                <div class="container pb-16 md:pb-20">
                    <div class="flex flex-col items-center sm:items-center sm:mx-auto max-w-148">
                        <h2 class="text-3xl font-medium text-center sm:text-center lg:text-5xl font-family-medium [word-spacing:0.02em]">
                            "Thousands of Icons"
                        </h2>
                        <p class="mt-3 text-center sm:text-center lg:mt-3 [word-spacing:0.02em] font-book text-muted-foreground">
                            "Elevate your designs with our curated icon library. Every icon is crafted with attention to detail, ensuring perfect
                            harmony in your interface."
                        </p>
                    </div>
                </div>
            </div>
            <div class="flex relative justify-center items-center py-16">
                <div class="flex flex-wrap gap-4 justify-center items-center max-w-xl">
                    <IconWrapper>
                        <Circle />
                    </IconWrapper>
                    <IconWrapper>
                        <Square />
                    </IconWrapper>
                    <IconWrapper>
                        <Package />
                    </IconWrapper>
                    <IconWrapper>
                        <Component />
                    </IconWrapper>
                    <IconWrapper>
                        <Network />
                    </IconWrapper>
                    <div class="flex justify-center items-center mx-4">
                        <Image
                            src="/icons/logo-light-square-88.png"
                            alt="Rust/UI logo"
                            width=88
                            height=88
                            class="rounded-lg shadow-lg dark:hidden"
                        />
                        <Image
                            src="/icons/logo-dark-square-88.png"
                            alt="Rust/UI logo"
                            width=88
                            height=88
                            class="hidden rounded-lg shadow-lg dark:block"
                        />
                    </div>
                    <IconWrapper>
                        <Boxes />
                    </IconWrapper>
                    <IconWrapper>
                        <Cuboid />
                    </IconWrapper>
                    <IconWrapper>
                        <Grid3X3 />
                    </IconWrapper>
                    <IconWrapper>
                        <Package />
                    </IconWrapper>
                    <IconWrapper>
                        <Component />
                    </IconWrapper>
                </div>

                <div class="flex relative flex-col gap-4 items-center mx-auto mt-8 max-w-96">
                    <p class="text-center font-book">
                        "A comprehensive icon solution built specifically for Rust web applications. No JavaScript dependencies, just pure Rust
                        components."
                    </p>
                </div>
            </div>
        </section>
    }
}
