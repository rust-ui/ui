use leptos::prelude::*;
use leptos_router::components::A;
use registry::ui::image::Image;

#[component]
pub fn LogoHomeLink() -> impl IntoView {
    view! {
        <A href="/">
            <Image
                src="/icons/logo-dark-square-48.webp"
                alt="Logo Rust/UI"
                width=48
                height=48
                priority=true
                class="hidden dark:block size-6"
            />
            <Image
                src="/icons/logo-light-square-48.webp"
                alt="Logo Rust/UI"
                width=48
                height=48
                priority=true
                class="dark:hidden size-6"
            />
        </A>
    }
}
