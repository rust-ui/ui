use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn CardCarousel(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!(
        "group rounded-[20px] overflow-hidden relative w-[320px] h-[320px] bg-gray-200",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { "data-name": "CardCarousel", class: "{c}", {children} } }
}

#[component]
pub fn CardCarouselOverlay(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!(
        "pb-4 absolute bottom-0 flex flex-col justify-between items-center z-10 h-[calc(50%+32px)] w-full",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { "data-name": "CardCarouselOverlay", class: "{c}", {children} } }
}

#[component]
pub fn CardCarouselNav(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!(
        "opacity-0 invisible group-hover:visible group-hover:opacity-100 transition-opacity duration-[240ms] p-3 flex justify-between items-center w-full",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { "data-name": "CardCarouselNav", class: "{c}", {children} } }
}

#[component]
pub fn CardCarouselNavButton(
    #[props(into, optional)] class: Option<String>,
    #[props(optional)] aria_disabled: bool,
    children: Element,
) -> Element {
    let c = tw_merge!(
        "border-0 rounded-full cursor-pointer flex items-center justify-center size-8 [&_svg:not([class*='size-'])]:size-3 bg-accent transition-all duration-[160ms] ease-in-out hover:shadow-sm hover:scale-110 aria-[disabled]:invisible",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        button {
            "data-name": "CardCarouselNavButton",
            class: "{c}",
            "aria-disabled": if aria_disabled { "true" } else { "false" },
            {children}
        }
    }
}

#[component]
pub fn CardCarouselIndicators(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!("gap-1 flex", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "CardCarouselIndicators", class: "{c}", {children} } }
}

#[component]
pub fn CardCarouselIndicator(
    #[props(into, optional)] class: Option<String>,
    #[props(optional)] aria_current: bool,
) -> Element {
    let c = tw_merge!(
        "rounded-full size-[6px] bg-white opacity-60 aria-[current]:opacity-100",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        span {
            "data-name": "CardCarouselIndicator",
            class: "{c}",
            "aria-current": if aria_current { "true" } else { "" },
        }
    }
}

#[component]
pub fn CardCarouselSlide(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!("snap-center shrink-0 w-full h-full", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "CardCarouselSlide", class: "{c}", {children} } }
}

#[component]
pub fn CardCarouselImage(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] src: Option<String>,
    #[props(into, optional)] alt: Option<String>,
) -> Element {
    let c = tw_merge!("object-cover w-full h-full", class.as_deref().unwrap_or(""));
    rsx! {
        img {
            "data-name": "CardCarouselImage",
            class: "{c}",
            src: src.unwrap_or_default(),
            alt: alt.unwrap_or_default(),
        }
    }
}

#[component]
pub fn CardCarouselTrack(children: Element) -> Element {
    rsx! {
        div {
            "data-name": "CardCarouselTrack",
            class: "flex overflow-x-scroll w-full h-full snap-x snap-mandatory scroll-smooth touch-pan-x [scrollbar-width:none] [&::-webkit-scrollbar]:hidden",
            {children}
        }
    }
}
