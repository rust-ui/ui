use dioxus::prelude::*;
use tw_merge::*;

#[component]
pub fn SvgIcon(
    class: std::option::Option<String>,
    data_name: std::option::Option<String>,
    children: Element,
) -> Element {
    let class = use_memo(move || tw_merge::tw_merge!("", class.as_deref().unwrap_or("")));

    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            "stroke-width": "2",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            class: "{class}",
            "data-name": data_name.as_deref().unwrap_or(""),
            {children}
        }
    }
}
