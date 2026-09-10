use dioxus::prelude::*;
use tw_merge::tw_merge;

// * Define your reusable Component here:
#[component]
pub fn MyButton(
    #[props(into, optional)] class: Option<String>,
    #[props(optional)] onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let merged = tw_merge!("px-4 py-2 bg-neutral-900 text-white rounded-md", class.as_deref().unwrap_or(""));
    rsx! {
        button {
            class: "{merged}",
            onclick: move |e| {
                if let Some(handler) = &onclick {
                    handler.call(e);
                }
            },
            {children}
        }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn DemoButtonWithClx() -> Element {
    let mut count_signal = use_signal(|| 0);
    let on_click = move |_| *count_signal.write() += 1;

    rsx! {
        MyButton { class: "bg-sky-500", onclick: on_click,
            "Click Me: "
            {count_signal().to_string()}
        }
    }
}
