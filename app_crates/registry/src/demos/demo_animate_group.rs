use icons::RotateCcw;
use leptos::prelude::*;

use crate::ui::animate::{AnimateGroup, AnimateGroupItem, AnimateVariant};
use crate::ui::button::{Button, ButtonSize, ButtonVariant};

// TODO DEMO. The trigger does not seem to work... 🤔 Figure out why

#[component]
pub fn DemoAnimateGroup() -> impl IntoView {
    let trigger_signal = RwSignal::new(false);

    let handle_click = move || {
        trigger_signal.update(|value| *value = !*value);
    };

    view! {
        <AnimateGroup class="flex relative flex-col gap-8 items-center w-full max-w-4xl h-80">
            <Button class="absolute top-0 right-0 z-10" on:click=move |_| handle_click() size=ButtonSize::Icon>
                <RotateCcw />
            </Button>

            <AnimateGroupItem variant=AnimateVariant::FadeUp delay_ms=250>
                <h1 class="mt-6 tracking-tighter text-center text-transparent bg-clip-text bg-gradient-to-r from-white to-gray-500">
                    "The only UI website"
                </h1>
            </AnimateGroupItem>
            <AnimateGroupItem variant=AnimateVariant::FadeUp class="mx-auto max-w-lg" delay_ms=450>
                <p class="text-base font-normal text-center text-neutral-300">
                    "Unstyled highly composable components that you can copy/paste in your own
                    codebase. Built with Tailwind CSS and a bit of Framer Motion. Customize them as
                    you want."
                </p>
            </AnimateGroupItem>

            <AnimateGroupItem variant=AnimateVariant::FadeUp class="space-x-4" delay_ms=600>
                <Button>"Components"</Button>
                <Button variant=ButtonVariant::Outline>"Get started"</Button>
            </AnimateGroupItem>
        </AnimateGroup>
    }
}
