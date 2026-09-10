use dioxus::prelude::*;
use tw_merge::tw_merge;

#[allow(dead_code)]
#[derive(Clone, PartialEq, Default)]
pub enum SparklesColor {
    #[default]
    Gray,
    Sky,
    Green,
    Orange,
}

#[allow(dead_code)]
#[derive(Clone, PartialEq, Default)]
pub enum SparklesDirection {
    #[default]
    Top,
    Bottom,
}

#[allow(dead_code)]
#[derive(Clone, PartialEq, Default)]
pub enum SparklesSize {
    #[default]
    Normal,
    Rounded,
}

const SPARKLES_BASE: &str = "relative h-80 w-full overflow-hidden [mask-image:radial-gradient(50%_50%,white,transparent)] before:absolute before:inset-0 before:opacity-80 after:absolute after:border-b after:border-2 after:border-input after:bg-neutral-300 dark:after:bg-neutral-900 after:-left-1/2 after:w-[200%]";

#[component]
pub fn SparklesEffect(
    #[props(default)] color: SparklesColor,
    #[props(default)] direction: SparklesDirection,
    #[props(default)] size: SparklesSize,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let dir_class = match direction {
        SparklesDirection::Top => "after:top-1/2 after:rounded-[50%]",
        SparklesDirection::Bottom => "after:bottom-1/2 after:rounded-[100%]",
    };
    let color_class = match color {
        SparklesColor::Gray => "before:bg-[radial-gradient(circle_at_bottom_center,#9C9DA1,transparent_90%)]",
        SparklesColor::Sky => "before:bg-[radial-gradient(circle_at_bottom_center,#369eff,transparent_90%)]",
        SparklesColor::Green => "before:bg-[radial-gradient(circle_at_bottom_center,#36b36f,transparent_90%)]",
        SparklesColor::Orange => "before:bg-[radial-gradient(circle_at_bottom_center,#e07b39,transparent_90%)]",
    };
    let size_class = match size {
        SparklesSize::Normal => "after:aspect-[1/0.8]",
        SparklesSize::Rounded => "after:aspect-[1/1.8]",
    };
    let merged = tw_merge!(SPARKLES_BASE, dir_class, color_class, size_class, class.as_deref().unwrap_or(""));

    rsx! {
        div { class: "{merged}", {children} }
    }
}

#[component]
pub fn SparklesSection(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("min-h-[500px] w-full overflow-hidden mx-auto", class.as_deref().unwrap_or(""));
    rsx! {
        div { class: "{merged}", {children} }
    }
}

#[component]
pub fn SparklesHeader(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "mx-auto w-full max-w-2xl relative z-10 flex flex-col gap-4 items-center justify-center",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        div { class: "{merged}", {children} }
    }
}

#[component]
pub fn SparklesDescription(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("w-full block text-center text-pretty px-2", class.as_deref().unwrap_or(""));
    rsx! {
        div { class: "{merged}", {children} }
    }
}
