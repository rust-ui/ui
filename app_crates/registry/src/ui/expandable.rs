use icons::X;
use leptos::prelude::*;
use tw_merge::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn ExpandableTrigger(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let merged_class = tw_merge!(
        "overflow-hidden relative rounded-lg bg-primary text-primary-foreground hover:bg-primary/90    h-[32px] w-[162px]    expandableMainWrapper ",
        class
    );

    view! {
        <div class=merged_class onclick="this.classList.add('expand')">
            {children()}
        </div>
    }
}

#[component]
pub fn ExpandableTransition(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let merged_class = tw_merge!("absolute transition-opacity duration-200 delay-100 from", class);

    view! {
        <div class=merged_class style="transition-timing-function: cubic-bezier(0.0, 0.0, 0.2, 1);">
            <div class="flex flex-row transition-transform duration-300 origin-top-left ease-[cubic-bezier(0.4,0.0,0.2,1)] from-contents">
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn ExpandableContent(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let merged_class = tw_merge!(
        "relative w-full bg-muted    h-full scale-[0.55] origin-top-left transition-transform duration-300 ease-[cubic-bezier(0.4,0.0,0.2,1)]  to-contents",
        class
    );

    view! {
        <div class="absolute w-full h-full opacity-0 transition-opacity duration-100 ease-[cubic-bezier(0.4,0.0,1,1)] to">
            // * 💁 "to" 👆
            <div class=merged_class>
                <button
                    type="button"
                    class="flex absolute top-1 right-1 justify-center items-center"
                    onclick="document.querySelector('.expandableMainWrapper').classList.remove('expand');event.stopPropagation();"
                >
                    <X class="size-6" />
                </button>
                {children()}
            </div>
        </div>
    }
}
