use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn TableWrapper(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("overflow-auto rounded-md border max-h-96", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "TableWrapper", class: "{merged}", {children} } }
}

#[component]
pub fn Table(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("w-full max-w-7xl text-sm caption-bottom", class.as_deref().unwrap_or(""));
    rsx! {
        table { class: "{merged}", {children} }
    }
}

#[component]
pub fn TableHeader(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("[&_tr]:border-b sticky top-0 z-10 bg-card", class.as_deref().unwrap_or(""));
    rsx! { thead { class: "{merged}", {children} } }
}

#[component]
pub fn TableBody(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("[&_tr:last-child]:border-0", class.as_deref().unwrap_or(""));
    rsx! { tbody { class: "{merged}", {children} } }
}

#[component]
pub fn TableRow(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] data_state: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "border-b transition-colors data-[state=selected]:bg-muted hover:bg-muted/50",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        tr {
            class: "{merged}",
            "data-state": data_state.as_deref(),
            {children}
        }
    }
}

#[component]
pub fn TableHead(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]",
        class.as_deref().unwrap_or("")
    );
    rsx! { th { class: "{merged}", {children} } }
}

#[component]
pub fn TableCell(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    // TODO(css-parity): leptos source has a malformed second selector here,
    // `&:has([role=checkbox])]:pl-3` is missing its leading `[`, so Tailwind emits
    // nothing for it. Kept byte-for-byte identical to leptos so both sites render
    // the same today. Looks like a bug in leptos: once leptos fixes it to
    // `[&:has([role=checkbox])]:pl-3`, fix this string too.
    let merged = tw_merge!(
        "p-4 align-middle [&:has([role=checkbox])]:pr-0  &:has([role=checkbox])]:pl-3",
        class.as_deref().unwrap_or("")
    );
    rsx! { td { class: "{merged}", {children} } }
}

#[component]
pub fn TableFooter(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged =
        tw_merge!("font-medium border border-t bg-muted/50 [&>tr]:last:border-b-0", class.as_deref().unwrap_or(""));
    rsx! { tfoot { class: "{merged}", {children} } }
}

#[component]
pub fn TableCaption(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("mt-4 text-sm text-muted-foreground", class.as_deref().unwrap_or(""));
    rsx! { caption { class: "{merged}", {children} } }
}
