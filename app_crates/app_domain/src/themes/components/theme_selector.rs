use icons::{Check, Copy};
use leptos::prelude::*;
use leptos_ui::clx;
use registry::hooks::use_copy_clipboard::use_copy_clipboard;
use registry::ui::button::{Button, ButtonSize, ButtonVariant};
use registry::ui::dialog::{Dialog, DialogBody, DialogContent, DialogTitle, DialogTrigger};
use registry::ui::scroll_area::ScrollArea;
use registry::ui::slider::Slider;
use wasm_bindgen::JsCast;

use super::oklch::Oklch;

const DEFAULT_L: f32 = 0.69;
const DEFAULT_C: f32 = 0.14;
const DEFAULT_H: f32 = 247.0;

const MAX_L: f32 = 1.0;
const MAX_C: f32 = 0.4;
const MAX_H: f32 = 360.0;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn ThemeSelector() -> impl IntoView {
    let l = RwSignal::new(DEFAULT_L);
    let c = RwSignal::new(DEFAULT_C);
    let h = RwSignal::new(DEFAULT_H);

    let color_primary_memo = Memo::new(move |_| Oklch::new(l(), c(), h()).to_oklch_string());

    let color_secondary_memo = Memo::new(move |_| {
        let primary = Oklch::new(l(), c(), h());
        let secondary = primary.secondary_with_factor(0.9);
        secondary.to_oklch_string()
    });

    // Memo for theme string using .replace
    let theme_memo = Memo::new(move |_| {
        let primary = color_primary_memo();
        let secondary = color_secondary_memo();
        THEME_TEMPLATE.replace("{primary}", &primary).replace("{secondary}", &secondary)
    });

    view! {
        <section class="w-full sm:w-96">
            <div class="flex flex-col gap-8 justify-center items-center">
                <RadiusSelector />
                <OklchSelector l=l c=c h=h color_primary_memo color_secondary_memo />
                // * Derive the theme signal
                <CopyCodeDialog theme=Signal::derive(theme_memo) />
            </div>
        </section>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn OklchSelector(
    l: RwSignal<f32>,
    c: RwSignal<f32>,
    h: RwSignal<f32>,
    #[prop(into)] color_primary_memo: Signal<String>,
    #[prop(into)] color_secondary_memo: Signal<String>,
) -> impl IntoView {
    Effect::new(move |_| {
        let primary = color_primary_memo();
        let secondary = color_secondary_memo();
        // Try to find .dark, else use documentElement
        let window = window();
        let Some(document) = window.document() else { return };
        let root = document.query_selector(".dark").ok().flatten().or_else(|| document.document_element());
        let Some(root) = root else { return };

        let Some(style) = root.dyn_ref::<web_sys::HtmlElement>().map(|el| el.style()) else { return };

        style.set_property("--primary", &primary).ok();
        style.set_property("--secondary", &secondary).ok();
        style.set_property("--selection", &primary).ok();
    });

    clx! {OklchTitle, small, "text-sm font-medium leading-none"}
    clx! {OklchNum, span, "ml-2 text-xs text-muted-foreground"}

    view! {
        <div class="w-full">
            <OklchTitle>"Lightness (L)"</OklchTitle>
            <OklchNum>{move || format!("({} / {})", l(), MAX_L)}</OklchNum>
            <Slider
                class="text-muted-foreground"
                attr:min="0"
                attr:max="1"
                attr:step="0.01"
                prop:value=l
                on:input=move |ev| {
                    let val = event_target_value(&ev).parse().unwrap_or(DEFAULT_L);
                    l.set(val);
                }
            />
        </div>
        <div class="w-full">
            <OklchTitle>"Chroma (C)"</OklchTitle>
            <OklchNum>{move || format!("({} / {})", c(), MAX_C)}</OklchNum>
            <Slider
                class="text-muted-foreground"
                attr:min="0"
                attr:max="0.4"
                attr:step="0.01"
                prop:value=c
                on:input=move |ev| {
                    let val = event_target_value(&ev).parse().unwrap_or(DEFAULT_C);
                    c.set(val);
                }
            />
        </div>
        <div class="w-full">
            <OklchTitle>"Hue (H)"</OklchTitle>
            <OklchNum>{move || format!("({} / {})", h(), MAX_H)}</OklchNum>
            <Slider
                class="text-muted-foreground"
                attr:min="0"
                attr:max="360"
                attr:step="1"
                prop:value=h
                on:input=move |ev| {
                    let val = event_target_value(&ev).parse().unwrap_or(DEFAULT_H);
                    h.set(val);
                }
            />
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn CopyCodeDialog(
    #[prop(into)] theme: Signal<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(default = ButtonVariant::Outline)] trigger_variant: ButtonVariant,
    #[prop(default = ButtonSize::Default)] trigger_size: ButtonSize,
) -> impl IntoView {
    let (copy_to_clipboard, copied) = use_copy_clipboard(Some(2000));

    let handle_copy = move |_| {
        copy_to_clipboard(&theme.get());
    };

    view! {
        <Dialog>
            <DialogTrigger variant=trigger_variant size=trigger_size>
                {match children {
                    Some(c) => c().into_any(),
                    None => view! { "Copy code" }.into_any(),
                }}
            </DialogTrigger>
            <DialogContent class="sm:max-w-[800px]">
                <DialogBody>
                    <DialogTitle>"Theme"</DialogTitle>
                    <p class="text-sm text-muted-foreground">Copy and paste the following code into your CSS file.</p>
                    <div class="flex flex-col gap-3">
                        <div class="overflow-hidden relative rounded-md h-[28rem]">
                            <ScrollArea class="w-full h-full rounded-[inherit]">
                                <div class="table p-2 min-w-full rounded-md bg-muted">
                                    <pre class="text-sm text-muted-foreground">
                                        <code>{theme}</code>
                                    </pre>
                                </div>
                            </ScrollArea>
                            <Button variant=ButtonVariant::Outline on:click=handle_copy class="absolute top-4 right-4">
                                {move || {
                                    if copied.get() {
                                        view! { <Check /> }.into_any()
                                    } else {
                                        view! { <Copy /> }.into_any()
                                    }
                                }}
                            </Button>
                        </div>
                    </div>
                </DialogBody>
            </DialogContent>
        </Dialog>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn RadiusSelector() -> impl IntoView {
    clx! {RadiusBtn, button, "inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors hover:bg-muted hover:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 data-[state=on]:bg-accent data-[state=on]:text-accent-foreground bg-transparent h-10 px-3"}

    view! {
        <div class="w-full">
            <small class="text-sm font-medium leading-none">Radius</small>
            <div
                role="group"
                dir="ltr"
                class="flex gap-1 justify-between items-center"
                tabindex="0"
                style="outline: none;"
            >
                <RadiusBtn
                    attr:data-state="off"
                    attr:role="radio"
                    attr:aria-checked="false"
                    class=""
                    attr:aria-label="Toggle radius 0"
                    attr:tabindex="-1"
                >
                    0
                </RadiusBtn>
                <RadiusBtn
                    attr:data-state="off"
                    attr:role="radio"
                    attr:aria-checked="false"
                    attr:aria-label="Toggle radius 0.3"
                    attr:tabindex="-1"
                >
                    0.3
                </RadiusBtn>
                <RadiusBtn
                    attr:data-state="on"
                    attr:role="radio"
                    attr:aria-checked="true"
                    attr:aria-label="Toggle radius 0.5"
                    attr:tabindex="-1"
                >
                    0.5
                </RadiusBtn>
                <RadiusBtn
                    attr:data-state="off"
                    attr:role="radio"
                    attr:aria-checked="false"
                    attr:aria-label="Toggle radius 0.75"
                    attr:tabindex="-1"
                >

                    0.75
                </RadiusBtn>
                <RadiusBtn
                    attr:data-state="off"
                    attr:role="radio"
                    attr:aria-checked="false"
                    attr:aria-label="Toggle radius 1.0"
                    attr:tabindex="-1"
                >
                    1.0
                </RadiusBtn>
            </div>
        </div>
    }
}

/* ========================================================== */
/*                      ✨ CONSTANTS ✨                       */
/* ========================================================== */

const THEME_TEMPLATE: &str = r#":root {
  --radius: 0.625rem;
  --primary: {primary};
  --primary-foreground: oklch(0.985 0 0);
  --secondary: {secondary};
  --secondary-foreground: oklch(0.205 0 0);
  --background: oklch(1 0 0);
  --foreground: oklch(0.145 0 0);
  --card: oklch(1 0 0);
  --card-foreground: oklch(0.145 0 0);
  --popover: oklch(1 0 0);
  --popover-foreground: oklch(0.145 0 0);
  --muted: oklch(0.97 0 0);
  --muted-foreground: oklch(0.556 0 0);
  --accent: oklch(0.97 0 0);
  --accent-foreground: oklch(0.205 0 0);
  --destructive: oklch(0.577 0.245 27.325);
  --success: oklch(0.65 0.16 145);
  --border: oklch(0.922 0 0);
  --input: oklch(0.922 0 0);
  --ring: oklch(0.708 0 0);
}

.dark {
  --primary: {primary};
  --primary-foreground: oklch(0.205 0 0);
  --secondary: {secondary};
  --secondary-foreground: oklch(0.985 0 0);
  --background: oklch(0.145 0 0);
  --foreground: oklch(0.985 0 0);
  --card: oklch(0.205 0 0);
  --card-foreground: oklch(0.985 0 0);
  --popover: oklch(0.205 0 0);
  --popover-foreground: oklch(0.985 0 0);
  --muted: oklch(0.269 0 0);
  --muted-foreground: oklch(0.708 0 0);
  --accent: oklch(0.269 0 0);
  --accent-foreground: oklch(0.985 0 0);
  --destructive: oklch(0.704 0.191 22.216);
  --success: oklch(0.65 0.16 145);
  --border: oklch(1 0 0 / 10%);
  --input: oklch(1 0 0 / 15%);
  --ring: oklch(0.556 0 0);
}"#;
