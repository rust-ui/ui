use icons::{Code, Eye, Terminal};
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::input_otp::{InputOTP, InputOTPGroup, InputOTPSlot};

const PAGE_CLASS: &str = "w-full";
const BLOCK_CLASS: &str = "block w-full space-y-3 mb-10 last:mb-0";
const TITLE_CLASS: &str = "text-2xl font-semibold tracking-tight";
const DESCRIPTION_CLASS: &str = "max-w-2xl text-sm leading-6 text-muted-foreground";
const ROW_CLASS: &str = "flex justify-between items-center";
const PREVIEW_CLASS: &str = "border rounded-xl w-full";
const CENTER_CLASS: &str = "flex justify-center items-center w-full min-h-[370px] px-4";
const CODE_BOX_CLASS: &str = "overflow-x-auto rounded-xl border bg-muted/40 p-4";
const CODE_TEXT_CLASS: &str = "text-sm leading-6 whitespace-pre-wrap text-foreground";
const TAB_BASE_CLASS: &str = "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all h-8 px-3 py-2 has-[>svg]:px-2.5 border outline-none";
const TAB_ACTIVE_CLASS: &str = "bg-secondary text-secondary-foreground shadow-xs";
const TAB_INACTIVE_CLASS: &str = "bg-background shadow-xs hover:bg-accent hover:text-accent-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/5";
const BASIC_CODE: &str = r#"use crate::ui::input_otp::{InputOTP, InputOTPGroup, InputOTPSlot};

view! {
    <InputOTP max_length=6>
        <InputOTPGroup>
            <InputOTPSlot index=0 />
            <InputOTPSlot index=1 />
            <InputOTPSlot index=2 />
            <InputOTPSlot index=3 />
            <InputOTPSlot index=4 />
            <InputOTPSlot index=5 />
        </InputOTPGroup>
    </InputOTP>
}"#;

const FOUR_DIGIT_CODE: &str = r#"use crate::ui::input_otp::{InputOTP, InputOTPGroup, InputOTPSlot};

view! {
    <InputOTP max_length=4>
        <InputOTPGroup>
            <InputOTPSlot index=0 />
            <InputOTPSlot index=1 />
            <InputOTPSlot index=2 />
            <InputOTPSlot index=3 />
        </InputOTPGroup>
    </InputOTP>
}"#;

const FILTER_CODE: &str = r#"let rejected = RwSignal::new(String::from("—"));

view! {
    <InputOTP max_length=6>
        <InputOTPGroup>
            <InputOTPSlot index=0 />
            <InputOTPSlot index=1 />
            <InputOTPSlot index=2 />
            <InputOTPSlot index=3 />
            <InputOTPSlot index=4 />
            <InputOTPSlot index=5 />
        </InputOTPGroup>
    </InputOTP>

    <p class="text-sm leading-6 text-muted-foreground">
        <span class="font-medium text-foreground/80">"Last rejected: "</span>
        {move || rejected.get()}
    </p>
}"#;

const FOCUS_CODE: &str = r#"use crate::ui::input_otp::{InputOTP, InputOTPGroup, InputOTPSlot};

view! {
    <InputOTP max_length=6>
        <InputOTPGroup>
            <InputOTPSlot index=0 />
            <InputOTPSlot index=1 />
            <InputOTPSlot index=2 />
            <InputOTPSlot index=3 />
            <InputOTPSlot index=4 />
            <InputOTPSlot index=5 />
        </InputOTPGroup>
    </InputOTP>
}"#;

const DYNAMIC_CODE: &str = r#"let show_dynamic = RwSignal::new(false);

view! {
    <Button on:click=move |_| show_dynamic.set(true)>"Append OTP"</Button>

    <Show when=move || show_dynamic.get()>
        <InputOTP max_length=6>
            <InputOTPGroup>
                <InputOTPSlot index=0 />
                <InputOTPSlot index=1 />
                <InputOTPSlot index=2 />
                <InputOTPSlot index=3 />
                <InputOTPSlot index=4 />
                <InputOTPSlot index=5 />
            </InputOTPGroup>
        </InputOTP>
    </Show>
}"#;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum DemoTab {
    #[default]
    Preview,
    Code,
}

#[component]
pub fn DemoInputOtp() -> impl IntoView {
    let show_dynamic = RwSignal::new(false);

    view! {
        <div class=PAGE_CLASS>
            <BasicBlock />
            <FourDigitBlock />
            <DigitFilterBlock />
            <FocusBlurBlock />
            <PostHydrationBlock show_dynamic=show_dynamic />
        </div>
    }
}

#[component]
fn BasicBlock() -> impl IntoView {
    let tab = RwSignal::new(DemoTab::Preview);

    view! {
        <section class=BLOCK_CLASS>
            <h3 class=TITLE_CLASS>"Basic OTP"</h3>
            <p class=DESCRIPTION_CLASS>
                "Six-slot OTP input controlled entirely in Rust. Type digits and watch the slots fill left to right."
            </p>
            <BlockToolbar tab=tab cli_label="demo_input_otp" />
            {move || {
                if tab.get() == DemoTab::Preview {
                    view! {
                        <PreviewShell>
                            <CenteredPreview>
                                <OtpInput
                                    max_length=6
                                    class="border-input/80 bg-background/20 data-[active=true]:border-ring data-[active=true]:bg-accent/30 data-[active=true]:ring-[3px] data-[active=true]:ring-ring/20"
                                />
                            </CenteredPreview>
                        </PreviewShell>
                    }
                        .into_any()
                } else {
                    view! { <CodePanel code=BASIC_CODE /> }.into_any()
                }
            }}
        </section>
    }
}

#[component]
fn FourDigitBlock() -> impl IntoView {
    let tab = RwSignal::new(DemoTab::Preview);

    view! {
        <section class=BLOCK_CLASS>
            <h3 class=TITLE_CLASS>"4-Digit Variant"</h3>
            <p class=DESCRIPTION_CLASS>
                "The same Rust controller wired to four-slot markup. Slot count comes from the DOM structure, not hardcoded."
            </p>
            <BlockToolbar tab=tab cli_label="demo_input_otp" />
            {move || {
                if tab.get() == DemoTab::Preview {
                    view! {
                        <PreviewShell>
                            <CenteredPreview>
                                <OtpInput
                                    max_length=4
                                    class="border-input/80 bg-background/20 data-[active=true]:border-ring data-[active=true]:bg-accent/30 data-[active=true]:ring-[3px] data-[active=true]:ring-ring/20"
                                />
                            </CenteredPreview>
                        </PreviewShell>
                    }
                        .into_any()
                } else {
                    view! { <CodePanel code=FOUR_DIGIT_CODE /> }.into_any()
                }
            }}
        </section>
    }
}

#[component]
fn DigitFilterBlock() -> impl IntoView {
    let tab = RwSignal::new(DemoTab::Preview);
    let rejected = RwSignal::new(String::from("—"));

    view! {
        <section class=BLOCK_CLASS>
            <h3 class=TITLE_CLASS>"Digit Filter"</h3>
            <p class=DESCRIPTION_CLASS>
                "Non-digit input is rejected before it reaches the hidden value. Type letters or symbols to see the filter in action."
            </p>
            <BlockToolbar tab=tab cli_label="demo_input_otp" />
            {move || {
                if tab.get() == DemoTab::Preview {
                    view! {
                        <div class="space-y-3">
                            <PreviewShell>
                                <CenteredPreview>
                                    <OtpInputWithRejected
                                        max_length=6
                                        rejected=rejected
                                        class="border-input/80 bg-background/20 data-[active=true]:border-ring data-[active=true]:bg-accent/30 data-[active=true]:ring-[3px] data-[active=true]:ring-ring/20"
                                    />
                                </CenteredPreview>
                            </PreviewShell>
                            <p class="text-sm leading-6 text-muted-foreground">
                                <span class="font-medium text-foreground/80">"Last rejected: "</span>
                                {move || rejected.get()}
                            </p>
                        </div>
                    }
                        .into_any()
                } else {
                    view! { <CodePanel code=FILTER_CODE /> }.into_any()
                }
            }}
        </section>
    }
}

#[component]
fn FocusBlurBlock() -> impl IntoView {
    let tab = RwSignal::new(DemoTab::Preview);

    view! {
        <section class=BLOCK_CLASS>
            <h3 class=TITLE_CLASS>"Focus and Blur"</h3>
            <p class=DESCRIPTION_CLASS>
                "The active slot highlight tracks the cursor and clears on blur. Click a slot to focus, click outside to blur."
            </p>
            <BlockToolbar tab=tab cli_label="demo_input_otp" />
            {move || {
                if tab.get() == DemoTab::Preview {
                    view! {
                        <PreviewShell>
                            <CenteredPreview>
                                <OtpInput
                                    max_length=6
                                    class="border-input/80 bg-background/20 data-[active=true]:border-ring data-[active=true]:bg-accent/30 data-[active=true]:ring-[3px] data-[active=true]:ring-ring/20"
                                />
                            </CenteredPreview>
                        </PreviewShell>
                    }
                        .into_any()
                } else {
                    view! { <CodePanel code=FOCUS_CODE /> }.into_any()
                }
            }}
        </section>
    }
}

#[component]
fn PostHydrationBlock(show_dynamic: RwSignal<bool>) -> impl IntoView {
    let tab = RwSignal::new(DemoTab::Preview);

    view! {
        <section class=BLOCK_CLASS>
            <h3 class=TITLE_CLASS>"Post-Hydration Init"</h3>
            <p class=DESCRIPTION_CLASS>
                "A new OTP root is injected after page load. The MutationObserver initializes it immediately without a reload."
            </p>
            <BlockToolbar tab=tab cli_label="demo_input_otp" />
            {move || {
                if tab.get() == DemoTab::Preview {
                    view! {
                        <PreviewShell>
                            <div class="flex flex-col items-center justify-center w-full min-h-[370px] px-4 py-8">
                                <Button class="w-fit" on:click=move |_| show_dynamic.set(true)>
                                    "Append OTP"
                                </Button>
                                <Show when=move || show_dynamic.get()>
                                    <div class="mt-8 pt-8 border-t w-full flex justify-center items-center">
                                        <OtpInput
                                            max_length=6
                                            class="border-input/80 bg-background/20 data-[active=true]:border-ring data-[active=true]:bg-accent/30 data-[active=true]:ring-[3px] data-[active=true]:ring-ring/20"
                                        />
                                    </div>
                                </Show>
                            </div>
                        </PreviewShell>
                    }
                        .into_any()
                } else {
                    view! { <CodePanel code=DYNAMIC_CODE /> }.into_any()
                }
            }}
        </section>
    }
}

#[component]
fn BlockToolbar(tab: RwSignal<DemoTab>, cli_label: &'static str) -> impl IntoView {
    view! {
        <div class=ROW_CLASS>
            <div class="flex items-center gap-2">
                <button
                    type="button"
                    class=move || format!(
                        "{} {}",
                        TAB_BASE_CLASS,
                        if tab.get() == DemoTab::Preview { TAB_ACTIVE_CLASS } else { TAB_INACTIVE_CLASS }
                    )
                    on:click=move |_| tab.set(DemoTab::Preview)
                >
                    <Eye />
                    <span>"Preview"</span>
                </button>
                <button
                    type="button"
                    class=move || format!(
                        "{} {}",
                        TAB_BASE_CLASS,
                        if tab.get() == DemoTab::Code { TAB_ACTIVE_CLASS } else { TAB_INACTIVE_CLASS }
                    )
                    on:click=move |_| tab.set(DemoTab::Code)
                >
                    <Code />
                    <span>"Code"</span>
                </button>
            </div>

            <div class="flex gap-2 items-center">
                <Button class="hidden sm:flex" variant=ButtonVariant::Outline size=ButtonSize::Sm>
                    <Terminal />
                    <span>{format!("ui add {cli_label}")}</span>
                </Button>
            </div>
        </div>
    }
}

#[component]
fn PreviewShell(children: Children) -> impl IntoView {
    view! {
        <div class=PREVIEW_CLASS>
            {children()}
        </div>
    }
}

#[component]
fn CenteredPreview(children: Children) -> impl IntoView {
    view! {
        <div class=CENTER_CLASS>
            {children()}
        </div>
    }
}

#[component]
fn CodePanel(code: &'static str) -> impl IntoView {
    view! {
        <div class=CODE_BOX_CLASS>
            <pre class=CODE_TEXT_CLASS><code>{code}</code></pre>
        </div>
    }
}

#[component]
fn OtpInput(max_length: usize, class: &'static str) -> impl IntoView {
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

#[component]
fn OtpInputWithRejected(max_length: usize, rejected: RwSignal<String>, class: &'static str) -> impl IntoView {
    let root_ref = NodeRef::<leptos::html::Div>::new();
    use_rejected_input_sync(root_ref, rejected);

    view! {
        <div node_ref=root_ref>
            <OtpInput max_length=max_length class=class />
        </div>
    }
}

fn use_rejected_input_sync(root_ref: NodeRef<leptos::html::Div>, rejected: RwSignal<String>) {
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

        let target: web_sys::EventTarget = input.unchecked_into();
        let listener = Closure::wrap(Box::new(move |event: web_sys::Event| {
            let Some(input_event) = event.dyn_ref::<web_sys::InputEvent>() else {
                return;
            };

            if input_event.input_type() != "insertText" {
                return;
            }

            let Some(data) = input_event.data() else { return };
            if data.chars().all(|ch| ch.is_ascii_digit()) {
                return;
            }

            rejected.set(data);
        }) as Box<dyn FnMut(web_sys::Event)>);

        let _ = target.add_event_listener_with_callback("beforeinput", listener.as_ref().unchecked_ref());
        listener.forget();
    });
}
