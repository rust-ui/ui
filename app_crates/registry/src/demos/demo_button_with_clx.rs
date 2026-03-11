use leptos::prelude::*;
use leptos_ui::clx;

// * 💁 Define your reusable Component here:
clx! {MyButton, button, "px-4 py-2 bg-neutral-900 text-white rounded-md"}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn DemoButtonWithClx() -> impl IntoView {
    let count_signal = RwSignal::new(0);
    let on_click = move |_| *count_signal.write() += 1;

    view! {
        <MyButton class="bg-sky-500" on:click=on_click>
            "Click Me: "
            {count_signal}
        </MyButton>
    }
}
