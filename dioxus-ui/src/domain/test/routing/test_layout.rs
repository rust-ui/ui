use dioxus::prelude::*;

use crate::Route;
use crate::components::navigation::header_docs::HeaderDocs;

#[component]
pub fn TestLayout() -> Element {
    rsx! {
        HeaderDocs {}
        div { class: "container flex flex-col gap-10 py-10",
            Outlet::<Route> {}
        }
    }
}
