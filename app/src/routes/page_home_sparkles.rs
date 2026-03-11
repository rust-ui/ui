use leptos::prelude::*;
use tw_merge::*;

#[derive(TwClass, Clone, Copy)]
#[tw(
    class = "relative h-80 w-full overflow-hidden  [mask-image:radial-gradient(50%_50%,white,transparent)] before:absolute before:inset-0  before:opacity-80 after:absolute after:border-b after:border-2 after:border-input   after:bg-neutral-300  dark:after:bg-neutral-900 after:-left-1/2 after:w-[200%]"
)]
pub struct SparklesClass {
    pub direction: SparklesDirection,
    pub color: SparklesColor,
    pub size: SparklesSize,
}

#[derive(TwVariant)]
pub enum SparklesDirection {
    #[tw(default, class = "after:top-1/2 after:rounded-[50%]")]
    Top,
    #[tw(class = "after:bottom-1/2 after:rounded-[100%]")]
    Bottom,
}

#[derive(TwVariant)]
pub enum SparklesColor {
    #[tw(default, class = "before:bg-[radial-gradient(circle_at_bottom_center,#9C9DA1,transparent_90%)]")]
    Gray,
    #[tw(class = "before:bg-[radial-gradient(circle_at_bottom_center,#369eff,transparent_90%)]")]
    Sky,
    #[tw(class = "before:bg-[radial-gradient(circle_at_bottom_center,#36b36f,transparent_90%)]")]
    Green,
    #[tw(class = "before:bg-[radial-gradient(circle_at_bottom_center,#e07b39,transparent_90%)]")]
    Orange,
}

#[derive(TwVariant)]
pub enum SparklesSize {
    #[tw(default, class = "after:aspect-[1/0.8]")]
    Normal,
    #[tw(class = "after:aspect-[1/1.8]")]
    Rounded,
}

#[component]
pub fn SparklesEffect(
    #[prop(into, optional)] direction: Signal<SparklesDirection>,
    #[prop(into, optional)] color: Signal<SparklesColor>,
    #[prop(into, optional)] size: Signal<SparklesSize>,
    #[prop(into, optional)] class: String,

    children: Children,
) -> impl IntoView {
    let merged_class = move || {
        let sparkles = SparklesClass { direction: direction.get(), color: color.get(), size: size.get() };
        sparkles.with_class(class.clone())
    };

    view! { <div class=merged_class>{children()}</div> }
}

#[component]
pub fn SparklesSection(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let merged_class = tw_merge!("min-h-[500px] w-full overflow-hidden mx-auto", class);

    view! { <div class=merged_class>{children()}</div> }
}

#[component]
pub fn SparklesHeader(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let merged_class =
        tw_merge!("mx-auto w-full max-w-2xl relative z-10  flex flex-col gap-4 items-center justify-center", class);

    view! { <div class=merged_class>{children()}</div> }
}

#[component]
pub fn SparklesDescription(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let merged_class = tw_merge!("w-full block text-center text-pretty px-2", class);

    view! { <div class=merged_class>{children()}</div> }
}
