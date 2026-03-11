use leptos::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Image(
    #[prop(into)] src: String,
    #[prop(into)] alt: String,
    width: u32,
    height: u32,
    #[prop(optional, into)] class: String,
    #[prop(into, default = "lazy".to_string())] loading: String,
    #[prop(into, default = "async".to_string())] decoding: String,
    #[prop(optional, into)] srcset: String,
    #[prop(optional, into)] sizes: String,
    #[prop(default = false)] priority: bool,
) -> impl IntoView {
    let loading_attr = if priority { "eager".to_string() } else { loading };
    let fetchpriority_attr = if priority { Some("high") } else { None };
    let merged_class = tw_merge!(class);

    view! {
        <img
            src=src
            alt=alt
            class=merged_class
            width=width
            height=height
            loading=loading_attr
            decoding=decoding
            fetchpriority=fetchpriority_attr
            srcset=srcset
            sizes=sizes
        />
    }
}
