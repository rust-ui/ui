use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::tw_merge;

mod components {
    use super::*;
    clx! {SwitchLabel, span, "text-sm font-medium"}
}

pub use components::*;

#[component]
pub fn Switch(
    #[prop(optional, into)] id: String,
    #[prop(optional, default = false)] checked: bool,
    #[prop(into, optional, default = "Toggle switch".to_string())] aria_label: String,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let is_checked = RwSignal::new(checked);
    let state = move || if is_checked.get() { "checked" } else { "unchecked" };

    let track_class = tw_merge!(
        "inline-flex h-6 w-11 shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary data-[state=unchecked]:bg-input",
        class
    );

    view! {
        <button
            data-name="Switch"
            id=id
            type="button"
            role="switch"
            aria-checked=move || is_checked.get().to_string()
            aria-label=aria_label
            data-state=state
            class=track_class
            on:click=move |_| is_checked.update(|c| *c = !*c)
        >
            <span
                data-state=state
                class="block rounded-full ring-0 shadow-lg transition-transform pointer-events-none size-5 bg-background data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0"
            />
        </button>
    }
}
