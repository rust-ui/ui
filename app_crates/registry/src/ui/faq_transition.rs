use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::*;

mod components {
    use super::*;
    clx! {Faq, div, "flex flex-col gap-3 w-full max-w-screen-md"}
    clx! {FaqTitle, span, "text-lg text-primary"}
    clx! {FaqDescription, p, "pr-6 mb-2 text-muted-foreground"}
    clx! {FaqSection, div, "w-full rounded bg-accent/30 hover:bg-accent flex flex-col"}

    clx! {RootContent, div, "grid overflow-hidden mt-2 transition-all duration-500 grid-rows-[0fr] peer-checked:grid-rows-[1fr]"}
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn FaqContent(children: Children) -> impl IntoView {
    view! {
        <RootContent>
            <div class="px-4 min-h-[0]">{children()}</div>
        </RootContent>
    }
}

#[component]
pub fn FaqLabel(
    #[prop(into, optional)] class: String,
    #[prop(into)] for_attr: &'static str,
    children: Children,
) -> impl IntoView {
    let merged_class = tw_merge!("flex justify-between items-center py-2 px-4 mt-2 w-full cursor-pointer", class);

    view! {
        <label class=merged_class for=for_attr>
            {children()}
        </label>
    }
}

#[component]
pub fn FaqInput(#[prop(into)] id: &'static str) -> impl IntoView {
    view! { <input id=id type="checkbox" class="ml-auto peer" /> } // sr-only
}
