use leptos::prelude::*;

#[component]
pub fn Steps(children: Children) -> impl IntoView {
    view! { <div class="pl-8 mb-12 ml-4 border-l border-border [counter-reset:step]">{children()}</div> }
}

#[component]
pub fn Step(children: Children) -> impl IntoView {
    view! {
        <h3 class="relative mt-8 text-xl font-semibold tracking-tight font-heading scroll-m-20 [counter-increment:step] before:content-[counter(step)] before:absolute before:inline-flex before:size-9 before:items-center before:justify-center before:rounded-full before:border-4 before:border-background before:bg-muted before:text-center before:font-mono before:text-base before:font-medium before:-ml-[50px] before:-mt-1 before:z-10">
            {children()}
        </h3>
    }
}
