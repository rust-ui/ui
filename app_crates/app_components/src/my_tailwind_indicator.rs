use leptos::prelude::*;

#[component]
pub fn MyTailwindIndicator() -> impl IntoView {
    let show_signal = RwSignal::new(false);

    #[cfg(not(feature = "ssr"))]
    Effect::new(move |_| {
        // Check if we're running on localhost
        let window = leptos::prelude::window();
        let location = window.location();
        let hostname = location.hostname().unwrap_or_default();

        if hostname == "localhost" || hostname == "127.0.0.1" {
            show_signal.set(true);
        }
    });

    view! {
        <Show when=move || show_signal.get() fallback=|| ()>
            <div
                data-name="TailwindIndicator"
                class="flex fixed bottom-3 left-3 justify-center items-center text-xs text-white rounded-full z-999 bg-primary size-6"
            >
                <div class="block sm:hidden">xs</div>
                <div class="hidden sm:block md:hidden">sm</div>
                <div class="hidden md:block lg:hidden">md</div>
                <div class="hidden lg:block xl:hidden">lg</div>
                <div class="hidden xl:block 2xl:hidden">xl</div>
                <div class="hidden 2xl:block">2xl</div>
            </div>
        </Show>
    }
}
