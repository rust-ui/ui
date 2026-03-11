---
title: "Carousel"
name: "carousel"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/carousel.rs"
description: "Rust/UI component for cycling through elements — slides, images, or cards — with prev/next navigation."
tags: []
---

# Carousel

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add carousel
```

## Component Code

```rust
use icons::{ChevronLeft, ChevronRight};
use leptos::context::Provider;
use leptos::prelude::*;
use tw_merge::*;

use crate::components::hooks::use_random::use_random_id_for;

/* ========================================================== */
/*                     CAROUSEL CONTEXT                       */
/* ========================================================== */

#[derive(Clone, Copy, PartialEq, Default)]
pub enum CarouselOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Clone)]
struct CarouselContext {
    carousel_id: String,
    orientation: CarouselOrientation,
}

/* ========================================================== */
/*                     CAROUSEL ROOT                          */
/* ========================================================== */

#[component]
pub fn Carousel(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(optional)] orientation: CarouselOrientation,
    #[prop(optional)] looping: bool,
) -> impl IntoView {
    let carousel_id = use_random_id_for("carousel");
    let ctx = CarouselContext { carousel_id: carousel_id.clone(), orientation };

    let orientation_str = match orientation {
        CarouselOrientation::Horizontal => "horizontal",
        CarouselOrientation::Vertical => "vertical",
    };

    let class = tw_merge!("relative", class);

    view! {
        <Provider value=ctx>
            <style>"[data-carousel-scroll]::-webkit-scrollbar { display: none; }"</style>

            <div
                data-name="Carousel"
                data-carousel-id=carousel_id.clone()
                data-carousel-orientation=orientation_str
                data-carousel-loop=looping.to_string()
                class=class
                role="region"
                aria-roledescription="carousel"
                tabindex="0"
            >
                {children()}
            </div>

            <script>
                {format!(
                    r#"
                    (function() {{
                        const setup = () => {{
                            const root = document.querySelector('[data-carousel-id="{0}"]');
                            const scrollEl = root && root.querySelector('[data-carousel-scroll="{0}"]');
                            const prevBtn = root && root.querySelector('[data-carousel-prev="{0}"]');
                            const nextBtn = root && root.querySelector('[data-carousel-next="{0}"]');

                            if (!root || !scrollEl) {{ setTimeout(setup, 50); return; }}
                            if (root.hasAttribute('data-carousel-initialized')) return;
                            root.setAttribute('data-carousel-initialized', 'true');

                            const isHorizontal = root.getAttribute('data-carousel-orientation') !== 'vertical';
                            const isLoop = root.getAttribute('data-carousel-loop') === 'true';

                            const getScrollPos  = () => isHorizontal ? scrollEl.scrollLeft : scrollEl.scrollTop;
                            const getScrollSize = () => isHorizontal ? scrollEl.scrollWidth  : scrollEl.scrollHeight;
                            const getClientSize = () => isHorizontal ? scrollEl.clientWidth  : scrollEl.clientHeight;

                            const canPrev = () => getScrollPos() > 1;
                            const canNext = () => Math.round(getScrollPos() + getClientSize()) < getScrollSize() - 1;

                            const countSlides = () => scrollEl.querySelectorAll('[role="group"]').length;
                            const currentSlide = () => {{
                                const size = getClientSize();
                                if (size === 0) return 1;
                                return Math.min(Math.round(getScrollPos() / size) + 1, countSlides());
                            }};

                            const updateIndicator = () => {{
                                const indicator = root.querySelector('[data-carousel-indicator="{0}"]');
                                if (!indicator) return;
                                indicator.textContent = `${{currentSlide()}} / ${{countSlides()}}`;
                            }};

                            const updateButtons = () => {{
                                if (prevBtn) prevBtn.disabled = !isLoop && !canPrev();
                                if (nextBtn) nextBtn.disabled = !isLoop && !canNext();
                                updateIndicator();
                            }};

                            const scrollPrev = () => {{
                                if (isLoop && !canPrev()) {{
                                    if (isHorizontal) scrollEl.scrollLeft = scrollEl.scrollWidth;
                                    else scrollEl.scrollTop = scrollEl.scrollHeight;
                                }} else {{
                                    const size = getClientSize();
                                    if (isHorizontal) scrollEl.scrollBy({{ left: -size, behavior: 'smooth' }});
                                    else scrollEl.scrollBy({{ top: -size, behavior: 'smooth' }});
                                }}
                            }};

                            const scrollNext = () => {{
                                if (isLoop && !canNext()) {{
                                    if (isHorizontal) scrollEl.scrollLeft = 0;
                                    else scrollEl.scrollTop = 0;
                                }} else {{
                                    const size = getClientSize();
                                    if (isHorizontal) scrollEl.scrollBy({{ left: size, behavior: 'smooth' }});
                                    else scrollEl.scrollBy({{ top: size, behavior: 'smooth' }});
                                }}
                            }};

                            if (prevBtn) prevBtn.addEventListener('click', scrollPrev);
                            if (nextBtn) nextBtn.addEventListener('click', scrollNext);

                            scrollEl.addEventListener('scroll', updateButtons, {{ passive: true }});

                            root.addEventListener('keydown', (e) => {{
                                if (isHorizontal) {{
                                    if (e.key === 'ArrowLeft')  {{ e.preventDefault(); scrollPrev(); }}
                                    else if (e.key === 'ArrowRight') {{ e.preventDefault(); scrollNext(); }}
                                }} else {{
                                    if (e.key === 'ArrowUp')   {{ e.preventDefault(); scrollPrev(); }}
                                    else if (e.key === 'ArrowDown') {{ e.preventDefault(); scrollNext(); }}
                                }}
                            }});

                            updateButtons();
                        }};

                        if (document.readyState === 'loading') {{
                            document.addEventListener('DOMContentLoaded', setup);
                        }} else {{
                            setup();
                        }}
                    }})();
                    "#,
                    carousel_id,
                )}
            </script>
        </Provider>
    }
}

/* ========================================================== */
/*                     CAROUSEL CONTENT                       */
/* ========================================================== */

#[component]
pub fn CarouselContent(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let ctx = expect_context::<CarouselContext>();
    let carousel_id = ctx.carousel_id;

    let (scroll_class, inner_class) = match ctx.orientation {
        CarouselOrientation::Horizontal => {
            ("overflow-x-auto snap-x snap-mandatory scroll-smooth", tw_merge!("flex -ml-4", class))
        }
        CarouselOrientation::Vertical => {
            ("overflow-y-auto snap-y snap-mandatory scroll-smooth", tw_merge!("flex flex-col -mt-4", class))
        }
    };

    view! {
        <div
            data-carousel-scroll=carousel_id
            class=scroll_class
            style="scrollbar-width: none; -ms-overflow-style: none;"
        >
            <div class=inner_class>{children()}</div>
        </div>
    }
}

/* ========================================================== */
/*                     CAROUSEL ITEM                          */
/* ========================================================== */

#[component]
pub fn CarouselItem(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let ctx = expect_context::<CarouselContext>();

    let padding = match ctx.orientation {
        CarouselOrientation::Horizontal => "pl-4",
        CarouselOrientation::Vertical => "pt-4",
    };

    let class = tw_merge!("min-w-0 shrink-0 grow-0 basis-full snap-start", padding, class);

    view! {
        <div data-name="CarouselItem" role="group" aria-roledescription="slide" class=class>
            {children()}
        </div>
    }
}

/* ========================================================== */
/*                     CAROUSEL PREVIOUS                      */
/* ========================================================== */

#[component]
pub fn CarouselPrevious(#[prop(optional, into)] class: String) -> impl IntoView {
    let ctx = expect_context::<CarouselContext>();

    let position_class = match ctx.orientation {
        CarouselOrientation::Horizontal => "top-1/2 -left-12 -translate-y-1/2",
        CarouselOrientation::Vertical => "-top-12 left-1/2 -translate-x-1/2 rotate-90",
    };

    let class = tw_merge!(
        "absolute inline-flex items-center justify-center size-8 rounded-full border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground cursor-pointer touch-manipulation disabled:pointer-events-none disabled:opacity-50",
        position_class,
        class
    );

    view! {
        <button data-name="CarouselPrevious" data-carousel-prev=ctx.carousel_id class=class aria-label="Previous slide">
            <ChevronLeft class="size-4" />
            <span class="sr-only">"Previous slide"</span>
        </button>
    }
}

/* ========================================================== */
/*                     CAROUSEL NEXT                          */
/* ========================================================== */

#[component]
pub fn CarouselNext(#[prop(optional, into)] class: String) -> impl IntoView {
    let ctx = expect_context::<CarouselContext>();

    let position_class = match ctx.orientation {
        CarouselOrientation::Horizontal => "top-1/2 -right-12 -translate-y-1/2",
        CarouselOrientation::Vertical => "-bottom-12 left-1/2 -translate-x-1/2 rotate-90",
    };

    let class = tw_merge!(
        "absolute inline-flex items-center justify-center size-8 rounded-full border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground cursor-pointer touch-manipulation disabled:pointer-events-none disabled:opacity-50",
        position_class,
        class
    );

    view! {
        <button data-name="CarouselNext" data-carousel-next=ctx.carousel_id class=class aria-label="Next slide">
            <ChevronRight class="size-4" />
            <span class="sr-only">"Next slide"</span>
        </button>
    }
}

/* ========================================================== */
/*                     CAROUSEL INDICATOR                     */
/* ========================================================== */

/// Displays "current / total" slide count, updated automatically by JS.
#[component]
pub fn CarouselIndicator(#[prop(optional, into)] class: String) -> impl IntoView {
    let ctx = expect_context::<CarouselContext>();
    let class = tw_merge!("py-2 text-center text-sm text-muted-foreground", class);

    view! { <div data-name="CarouselIndicator" data-carousel-indicator=ctx.carousel_id class=class /> }
}
```
