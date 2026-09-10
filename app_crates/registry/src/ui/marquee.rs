use dioxus::prelude::*;
use tw_merge::tw_merge;

const MARQUEE_CSS: &str = "\
@keyframes marquee_horizontal {
  from { transform: translateX(0); }
  to { transform: translateX(calc(-100% - var(--gap))); }
}

@keyframes marquee_vertical {
  from { transform: translateY(0); }
  to { transform: translateY(calc(-100% - var(--gap))); }
}

.animate__marquee__row {
  animation-name: marquee_horizontal;
  animation-duration: var(--duration);
  animation-timing-function: linear;
  animation-iteration-count: infinite;
}

.group:hover .animate__marquee__row {
  animation-play-state: paused;
}";

#[component]
pub fn MarqueeRow(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!(
        "animate__marquee__row flex flex-row justify-around shrink-0 [gap:var(--gap)]",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { "data-name": "MarqueeRow", class: "{c}", {children} } }
}

#[component]
pub fn Marquee(children: Element) -> Element {
    rsx! {
        style { "{MARQUEE_CSS}" }
        div {
            "data-name": "Marquee",
            class: "flex overflow-hidden flex-row p-2 group [--gap:1rem] [gap:var(--gap)] [--duration:20s]",
            {children}
        }
    }
}

#[component]
pub fn MarqueeWrapper(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!(
        "flex overflow-hidden relative flex-col justify-center items-center p-20 w-full h-full md:shadow-xl min-h-[300px] bg-background",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        div { "data-name": "MarqueeWrapper", class: "{c}",
            {children}
            div { class: "pointer-events-none absolute inset-y-0 left-0 w-1/3 bg-gradient-to-r from-background to-transparent" }
            div { class: "pointer-events-none absolute inset-y-0 right-0 w-1/3 bg-gradient-to-l from-background to-transparent" }
        }
    }
}
