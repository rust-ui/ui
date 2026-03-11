use leptos::prelude::*;
use tw_merge::*;

#[component]
pub fn SvgIcon(
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] data_name: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <svg
            class=tw_merge!("", class)
            data-name=data_name
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            {children()}
        </svg>
    }
}
