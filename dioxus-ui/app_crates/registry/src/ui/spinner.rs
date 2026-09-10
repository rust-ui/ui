use dioxus::prelude::*;
use icons::{Loader, LoaderCircle};
use tw_merge::tw_merge;

#[component]
pub fn Spinner(#[props(into, optional)] class: Option<String>) -> Element {
    let merged_class = tw_merge!("size-4 animate-spin", class.as_deref().unwrap_or(""));

    rsx! {
        LoaderCircle { class: "{merged_class}" }
    }
}

#[component]
pub fn SpinnerCircle(#[props(into, optional)] class: Option<String>) -> Element {
    let merged_class = tw_merge!("size-4 animate-spin", class.as_deref().unwrap_or(""));

    rsx! {
        Loader { class: "{merged_class}" }
    }
}
