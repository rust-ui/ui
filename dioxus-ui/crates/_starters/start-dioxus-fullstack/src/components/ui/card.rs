use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Card(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("rounded-xl border bg-card text-card-foreground shadow", class.as_deref().unwrap_or(""));
    rsx! { div { class: "{merged}", {children} } }
}

#[component]
pub fn CardHeader(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("flex flex-col gap-1.5 p-6", class.as_deref().unwrap_or(""));
    rsx! { div { class: "{merged}", {children} } }
}

#[component]
pub fn CardTitle(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("font-semibold leading-none tracking-tight", class.as_deref().unwrap_or(""));
    rsx! { h3 { class: "{merged}", {children} } }
}

#[component]
pub fn CardDescription(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("text-sm text-muted-foreground", class.as_deref().unwrap_or(""));
    rsx! { p { class: "{merged}", {children} } }
}

#[component]
pub fn CardContent(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("p-6 pt-0", class.as_deref().unwrap_or(""));
    rsx! { div { class: "{merged}", {children} } }
}

#[component]
pub fn CardFooter(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("flex items-center p-6 pt-0", class.as_deref().unwrap_or(""));
    rsx! { div { class: "{merged}", {children} } }
}
