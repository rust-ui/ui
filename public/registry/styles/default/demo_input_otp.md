---
title: "Demo Input Otp"
name: "demo_input_otp"
cargo_dependencies: []
registry_dependencies: ["input_otp"]
type: "components:demos"
path: "demos/demo_input_otp.rs"
---

# Demo Input Otp

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_input_otp
```

## Component Code

```rust
use leptos::prelude::*;

use crate::ui::button::Button;
use crate::ui::input_otp::{InputOTP, InputOTPGroup, InputOTPSlot};

#[component]
pub fn DemoInputOtp() -> impl IntoView {
    let show_dynamic = RwSignal::new(false);

    view! {
        <div class="space-y-8">
            <DemoSection
                title="Basic OTP"
                description="A clean six-slot OTP with an optional slow-caret mode to make focus and active-state behavior obvious."
            >
                <OtpCard max_length=6 />
            </DemoSection>

            <DemoSection
                title="4-digit variant"
                description="The same controller running against shorter markup to prove slot count is driven by the DOM."
            >
                <OtpCard max_length=4 />
            </DemoSection>

            <DemoSection
                title="Digit filter"
                description="Type letters, symbols, or spaces. Invalid text is blocked before it ever reaches the OTP value."
            >
                <FilterCard />
            </DemoSection>

            <DemoSection
                title="Post-hydration init"
                description="Insert a brand-new OTP root after page load and verify that it becomes interactive immediately."
            >
                <DynamicCard show_dynamic=show_dynamic />
            </DemoSection>
        </div>
    }
}

#[component]
fn DemoSection(title: &'static str, description: &'static str, children: Children) -> impl IntoView {
    view! {
        <section class="rounded-[28px] border bg-card/65 px-6 py-6 shadow-sm sm:px-8 sm:py-8">
            <div class="space-y-6">
                <div class="space-y-2">
                    <h3 class="text-2xl font-semibold tracking-tight">{title}</h3>
                    <p class="max-w-2xl text-sm leading-6 text-muted-foreground">{description}</p>
                </div>
                {children()}
            </div>
        </section>
    }
}

#[component]
fn OtpCard(max_length: usize) -> impl IntoView {
    let slow_caret = RwSignal::new(false);

    view! {
        <div class="space-y-6">
            <div class="flex flex-wrap items-center gap-3">
                <label class="inline-flex items-center gap-3 rounded-full border bg-background/70 px-4 py-2 text-sm text-muted-foreground">
                    <input
                        type="checkbox"
                        class="size-4 rounded border-input"
                        prop:checked=move || slow_caret.get()
                        on:change=move |event| slow_caret.set(event_target_checked(&event))
                    />
                    "Slow caret"
                </label>
                <p class="text-sm text-muted-foreground">
                    {move || format!("{max_length} slots with active-state highlighting and native input behavior.")}
                </p>
            </div>

            <div class="rounded-3xl border bg-[radial-gradient(circle_at_top,rgba(255,255,255,0.05),rgba(255,255,255,0.015)_55%)] px-6 py-10 sm:px-8 sm:py-12">
                <div class="flex min-h-28 items-center justify-center">
                    <OtpMarkup max_length=max_length slow_caret=slow_caret />
                </div>
            </div>
        </div>
    }
}

#[component]
fn FilterCard() -> impl IntoView {
    let rejected = RwSignal::new(String::from("none"));

    view! {
        <div class="grid gap-5 xl:grid-cols-[minmax(0,1.25fr)_minmax(18rem,0.75fr)]">
            <div class="rounded-3xl border bg-[radial-gradient(circle_at_top,rgba(125,211,252,0.08),rgba(255,255,255,0.015)_55%)] px-6 py-10 sm:px-8 sm:py-12">
                <div class="flex min-h-28 items-center justify-center">
                    <OtpMarkupWithRejection max_length=6 rejected=rejected />
                </div>
            </div>

            <div class="rounded-3xl border bg-background/60 px-6 py-6">
                <p class="text-xs font-medium uppercase tracking-[0.18em] text-muted-foreground">"Last rejected input"</p>
                <p class="mt-5 break-all font-mono text-3xl font-semibold tracking-tight">{move || rejected.get()}</p>
                <p class="mt-4 text-sm leading-6 text-muted-foreground">
                    "Blocked characters never reach the OTP slots, so the UI stays digit-only."
                </p>
            </div>
        </div>
    }
}

#[component]
fn DynamicCard(show_dynamic: RwSignal<bool>) -> impl IntoView {
    view! {
        <div class="space-y-5">
            <div class="flex flex-wrap items-center gap-4">
                <Button on:click=move |_| show_dynamic.set(true) class="w-fit">
                    {move || if show_dynamic.get() { "Dynamic OTP Added" } else { "Append Dynamic OTP" }}
                </Button>
                <p class="text-sm text-muted-foreground">
                    "The newly inserted OTP should respond immediately without a page refresh."
                </p>
            </div>

            <Show
                when=move || show_dynamic.get()
                fallback=move || {
                    view! {
                        <div class="rounded-3xl border border-dashed bg-background/40 px-6 py-10 text-sm leading-6 text-muted-foreground">
                            "Press the button to append a new OTP widget into the DOM."
                        </div>
                    }
                }
            >
                <div class="rounded-3xl border bg-[radial-gradient(circle_at_top,rgba(74,222,128,0.06),rgba(255,255,255,0.015)_55%)] px-6 py-10 sm:px-8 sm:py-12">
                    <div class="flex min-h-28 items-center justify-center">
                        <OtpMarkup max_length=6 slow_caret=RwSignal::new(false) />
                    </div>
                </div>
            </Show>
        </div>
    }
}

#[component]
fn OtpMarkup(max_length: usize, slow_caret: RwSignal<bool>) -> impl IntoView {
    view! {
        <Show
            when=move || slow_caret.get()
            fallback=move || {
                view! {
                    <OtpSlots
                        max_length=max_length
                        class="border-input/80 bg-background/20 data-[active=true]:border-emerald-500 data-[active=true]:bg-emerald-500/10 data-[active=true]:ring-emerald-500/25"
                    />
                }
            }
        >
            <OtpSlots
                max_length=max_length
                class="border-sky-300/70 bg-background/20 data-[active=true]:border-sky-500 data-[active=true]:bg-sky-500/10 data-[active=true]:ring-sky-500/25 [&_[data-otp-caret]>div]:duration-[2200ms]"
            />
        </Show>
    }
}

#[component]
fn OtpMarkupWithRejection(max_length: usize, rejected: RwSignal<String>) -> impl IntoView {
    view! {
        <OtpInputWithHooks
            max_length=max_length
            class="border-sky-300/70 bg-background/20 data-[active=true]:border-sky-500 data-[active=true]:bg-sky-500/10 data-[active=true]:ring-sky-500/25 [&_[data-otp-caret]>div]:duration-[2200ms]"
            on_rejected=Some(rejected)
        />
    }
}

#[component]
fn OtpInputWithHooks(
    max_length: usize,
    class: &'static str,
    #[prop(optional)] on_rejected: Option<RwSignal<String>>,
) -> impl IntoView {
    let root_ref = NodeRef::<leptos::html::Div>::new();

    use_rejected_input_sync(root_ref, on_rejected);

    view! {
        <div node_ref=root_ref>
            <OtpSlots max_length=max_length class=class />
        </div>
    }
}

#[component]
fn OtpSlots(max_length: usize, class: &'static str) -> impl IntoView {
    view! {
        <InputOTP max_length=max_length as u32 class="w-full justify-center">
            <InputOTPGroup class="justify-center">
                <For
                    each=move || 0..max_length
                    key=|index| *index
                    children=move |index| view! { <InputOTPSlot index=index as u32 class=class /> }
                />
            </InputOTPGroup>
        </InputOTP>
    }
}

fn use_rejected_input_sync(root_ref: NodeRef<leptos::html::Div>, rejected: Option<RwSignal<String>>) {
    Effect::new(move |_| {
        let Some(root) = root_ref.get() else { return };
        if root.get_attribute("data-demo-otp-rejection").is_some() {
            return;
        }

        let Ok(Some(input)) = root.query_selector("input[data-otp-input]") else {
            return;
        };
        let Ok(input) = input.dyn_into::<web_sys::HtmlInputElement>() else {
            return;
        };
        let _ = root.set_attribute("data-demo-otp-rejection", "true");

        let Some(rejected_signal) = rejected else { return };
        let beforeinput_target: web_sys::EventTarget = input.unchecked_into();
        let rejected_listener = wasm_bindgen::closure::Closure::wrap(Box::new(move |event: web_sys::Event| {
            let Some(event) = event.dyn_ref::<web_sys::InputEvent>() else {
                return;
            };

            if event.input_type() != "insertText" {
                return;
            }

            let Some(data) = event.data() else { return };
            if data.chars().all(|ch| ch.is_ascii_digit()) {
                return;
            }

            rejected_signal.set(data);
        }) as Box<dyn FnMut(web_sys::Event)>);

        let _ = beforeinput_target
            .add_event_listener_with_callback("beforeinput", rejected_listener.as_ref().unchecked_ref());
        rejected_listener.forget();
    });
}
```
