use std::sync::atomic::{AtomicU64, Ordering};

use dioxus::prelude::*;
use icons::{ChevronLeft, ChevronRight};
use tw_merge::tw_merge;

static CAROUSEL_COUNTER: AtomicU64 = AtomicU64::new(0);

fn use_carousel_id() -> String {
    use_hook(|| {
        let id = CAROUSEL_COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("carousel_{id}")
    })
}

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

#[component]
pub fn Carousel(
    children: Element,
    #[props(into, optional)] class: Option<String>,
    #[props(default = CarouselOrientation::Horizontal)] orientation: CarouselOrientation,
    #[props(optional)] looping: bool,
) -> Element {
    let carousel_id = use_carousel_id();

    provide_context(CarouselContext { carousel_id: carousel_id.clone(), orientation });

    let orientation_str = match orientation {
        CarouselOrientation::Horizontal => "horizontal",
        CarouselOrientation::Vertical => "vertical",
    };

    let c = tw_merge!("relative", class.as_deref().unwrap_or(""));

    let script = format!(
        r#"(function() {{
        const setup = () => {{
            const root = document.querySelector('[data-carousel-id="{carousel_id}"]');
            const scrollEl = root && root.querySelector('[data-carousel-scroll="{carousel_id}"]');
            const prevBtn = root && root.querySelector('[data-carousel-prev="{carousel_id}"]');
            const nextBtn = root && root.querySelector('[data-carousel-next="{carousel_id}"]');
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
            const currentSlide = () => {{ const size = getClientSize(); if (size === 0) return 1; return Math.min(Math.round(getScrollPos() / size) + 1, countSlides()); }};
            const updateIndicator = () => {{ const indicator = root.querySelector('[data-carousel-indicator="{carousel_id}"]'); if (!indicator) return; indicator.textContent = `${{currentSlide()}} / ${{countSlides()}}`; }};
            const updateButtons = () => {{ if (prevBtn) prevBtn.disabled = !isLoop && !canPrev(); if (nextBtn) nextBtn.disabled = !isLoop && !canNext(); updateIndicator(); }};
            const scrollPrev = () => {{ if (isLoop && !canPrev()) {{ if (isHorizontal) scrollEl.scrollLeft = scrollEl.scrollWidth; else scrollEl.scrollTop = scrollEl.scrollHeight; }} else {{ const size = getClientSize(); if (isHorizontal) scrollEl.scrollBy({{ left: -size, behavior: 'smooth' }}); else scrollEl.scrollBy({{ top: -size, behavior: 'smooth' }}); }} }};
            const scrollNext = () => {{ if (isLoop && !canNext()) {{ if (isHorizontal) scrollEl.scrollLeft = 0; else scrollEl.scrollTop = 0; }} else {{ const size = getClientSize(); if (isHorizontal) scrollEl.scrollBy({{ left: size, behavior: 'smooth' }}); else scrollEl.scrollBy({{ top: size, behavior: 'smooth' }}); }} }};
            if (prevBtn) prevBtn.addEventListener('click', scrollPrev);
            if (nextBtn) nextBtn.addEventListener('click', scrollNext);
            scrollEl.addEventListener('scroll', updateButtons, {{ passive: true }});
            root.addEventListener('keydown', (e) => {{ if (isHorizontal) {{ if (e.key === 'ArrowLeft') {{ e.preventDefault(); scrollPrev(); }} else if (e.key === 'ArrowRight') {{ e.preventDefault(); scrollNext(); }} }} else {{ if (e.key === 'ArrowUp') {{ e.preventDefault(); scrollPrev(); }} else if (e.key === 'ArrowDown') {{ e.preventDefault(); scrollNext(); }} }} }});
            updateButtons();
        }};
        if (document.readyState === 'loading') {{ document.addEventListener('DOMContentLoaded', setup); }} else {{ setup(); }}
    }})();"#
    );

    rsx! {
        style { "[data-carousel-scroll]::-webkit-scrollbar {{ display: none; }}" }
        div {
            "data-name": "Carousel",
            "data-carousel-id": "{carousel_id}",
            "data-carousel-orientation": "{orientation_str}",
            "data-carousel-loop": "{looping}",
            class: "{c}",
            role: "region",
            "aria-roledescription": "carousel",
            tabindex: "0",
            {children}
        }
        script { "{script}" }
    }
}

#[component]
pub fn CarouselContent(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let ctx = use_context::<CarouselContext>();

    let (scroll_class, inner_class) = match ctx.orientation {
        CarouselOrientation::Horizontal => (
            "overflow-x-auto snap-x snap-mandatory scroll-smooth",
            tw_merge!("flex -ml-4", class.as_deref().unwrap_or("")),
        ),
        CarouselOrientation::Vertical => (
            "overflow-y-auto snap-y snap-mandatory scroll-smooth",
            tw_merge!("flex flex-col -mt-4", class.as_deref().unwrap_or("")),
        ),
    };

    rsx! {
        div {
            "data-carousel-scroll": "{ctx.carousel_id}",
            class: "{scroll_class}",
            style: "scrollbar-width: none; -ms-overflow-style: none;",
            div { class: "{inner_class}", {children} }
        }
    }
}

#[component]
pub fn CarouselItem(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let ctx = use_context::<CarouselContext>();

    let padding = match ctx.orientation {
        CarouselOrientation::Horizontal => "pl-4",
        CarouselOrientation::Vertical => "pt-4",
    };

    let c = tw_merge!("min-w-0 shrink-0 grow-0 basis-full snap-start", padding, class.as_deref().unwrap_or(""));

    rsx! {
        div {
            "data-name": "CarouselItem",
            role: "group",
            "aria-roledescription": "slide",
            class: "{c}",
            {children}
        }
    }
}

#[component]
pub fn CarouselPrevious(#[props(into, optional)] class: Option<String>) -> Element {
    let ctx = use_context::<CarouselContext>();

    let position_class = match ctx.orientation {
        CarouselOrientation::Horizontal => "top-1/2 -left-12 -translate-y-1/2",
        CarouselOrientation::Vertical => "-top-12 left-1/2 -translate-x-1/2 rotate-90",
    };

    let c = tw_merge!(
        "absolute inline-flex items-center justify-center size-8 rounded-full border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground cursor-pointer touch-manipulation disabled:pointer-events-none disabled:opacity-50",
        position_class,
        class.as_deref().unwrap_or("")
    );

    rsx! {
        button {
            "data-name": "CarouselPrevious",
            "data-carousel-prev": "{ctx.carousel_id}",
            class: "{c}",
            "aria-label": "Previous slide",
            ChevronLeft { class: "size-4" }
            span { class: "sr-only", "Previous slide" }
        }
    }
}

#[component]
pub fn CarouselNext(#[props(into, optional)] class: Option<String>) -> Element {
    let ctx = use_context::<CarouselContext>();

    let position_class = match ctx.orientation {
        CarouselOrientation::Horizontal => "top-1/2 -right-12 -translate-y-1/2",
        CarouselOrientation::Vertical => "-bottom-12 left-1/2 -translate-x-1/2 rotate-90",
    };

    let c = tw_merge!(
        "absolute inline-flex items-center justify-center size-8 rounded-full border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground cursor-pointer touch-manipulation disabled:pointer-events-none disabled:opacity-50",
        position_class,
        class.as_deref().unwrap_or("")
    );

    rsx! {
        button {
            "data-name": "CarouselNext",
            "data-carousel-next": "{ctx.carousel_id}",
            class: "{c}",
            "aria-label": "Next slide",
            ChevronRight { class: "size-4" }
            span { class: "sr-only", "Next slide" }
        }
    }
}

#[component]
pub fn CarouselIndicator(#[props(into, optional)] class: Option<String>) -> Element {
    let ctx = use_context::<CarouselContext>();
    let c = tw_merge!("py-2 text-center text-sm text-muted-foreground", class.as_deref().unwrap_or(""));

    rsx! {
        div {
            "data-name": "CarouselIndicator",
            "data-carousel-indicator": "{ctx.carousel_id}",
            class: "{c}",
        }
    }
}
