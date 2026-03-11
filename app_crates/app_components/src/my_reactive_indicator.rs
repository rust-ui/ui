use leptos::prelude::*;

#[cfg(not(feature = "ssr"))]
const TODO_TIMEOUT_MS: u64 = 100;
#[cfg(not(feature = "ssr"))]
const LOCALHOST: &str = "localhost";
#[cfg(not(feature = "ssr"))]
const LOCALHOST_IP: &str = "127.0.0.1";

#[component]
pub fn MyReactiveIndicator() -> impl IntoView {
    let show_signal = RwSignal::new(false);
    let is_ready_signal = RwSignal::new(false);

    #[cfg(not(feature = "ssr"))]
    Effect::new(move |_| {
        // Check if we're running on localhost
        let window = leptos::prelude::window();
        let location = window.location();
        let hostname = location.hostname().unwrap_or_default();

        if hostname == LOCALHOST || hostname == LOCALHOST_IP {
            show_signal.set(true);

            set_timeout(
                move || {
                    // Use try_set to avoid panic if component is unmounted
                    let _ = is_ready_signal.try_set(true);
                },
                std::time::Duration::from_millis(TODO_TIMEOUT_MS),
            );
        }
    });

    view! {
        <Show when=move || show_signal.get() fallback=|| ()>
            <div
                data-name="ReactiveIndicator"
                class=move || {
                    let base_class = "size-3 fixed bottom-3 right-5 z-999 rounded-full transition-colors duration-300 ease-in-out";
                    let color_class = if is_ready_signal.get() { "bg-green-500" } else { "bg-orange-500" };
                    format!("{base_class} {color_class}")
                }
            />
        </Show>
    }
}
