use dioxus::document::eval;
use dioxus::prelude::*;
use icons::{Check, Copy};
use registry::ui::button::{Button, ButtonSize, ButtonVariant};
use registry::ui::dialog::{Dialog, DialogBody, DialogContent, DialogTitle, DialogTrigger};
use registry::ui::scroll_area::ScrollArea;
use registry::ui::slider::Slider;

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
pub fn ThemeSelector() -> Element {
    let l = use_signal(|| DEFAULT_L);
    let c = use_signal(|| DEFAULT_C);
    let h = use_signal(|| DEFAULT_H);

    let color_primary_memo = use_memo(move || Oklch::new(l(), c(), h()).to_oklch_string());
    let color_secondary_memo = use_memo(move || Oklch::new(l(), c(), h()).secondary_with_factor(0.9).to_oklch_string());

    let theme_memo = use_memo(move || {
        THEME_TEMPLATE.replace("{primary}", &color_primary_memo()).replace("{secondary}", &color_secondary_memo())
    });

    rsx! {
        section { class: "w-full sm:w-96",
            div { class: "flex flex-col gap-8 justify-center items-center",
                RadiusSelector {}
                OklchSelector {
                    l,
                    c,
                    h,
                    color_primary_memo: ReadSignal::from(color_primary_memo),
                    color_secondary_memo: ReadSignal::from(color_secondary_memo),
                }
                CopyCodeDialog { theme: ReadSignal::from(theme_memo) }
            }
        }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn OklchSelector(
    mut l: Signal<f32>,
    mut c: Signal<f32>,
    mut h: Signal<f32>,
    color_primary_memo: ReadSignal<String>,
    color_secondary_memo: ReadSignal<String>,
) -> Element {
    use_effect(move || {
        let primary = color_primary_memo();
        let secondary = color_secondary_memo();
        eval(&format!(
            "var root = document.querySelector('.dark') || document.documentElement;\
             root.style.setProperty('--primary', '{primary}');\
             root.style.setProperty('--secondary', '{secondary}');\
             root.style.setProperty('--selection', '{primary}');"
        ));
    });

    rsx! {
        div { class: "w-full",
            small { class: "text-sm font-medium leading-none", "Lightness (L)" }
            span { class: "ml-2 text-xs text-muted-foreground", "({l} / {MAX_L})" }
            Slider {
                class: "text-muted-foreground",
                min: 0.0,
                max: 1.0,
                step: 0.01,
                value: l() as f64,
                oninput: move |e: FormEvent| {
                    let val = e.value().parse().unwrap_or(DEFAULT_L);
                    l.set(val);
                },
            }
        }
        div { class: "w-full",
            small { class: "text-sm font-medium leading-none", "Chroma (C)" }
            span { class: "ml-2 text-xs text-muted-foreground", "({c} / {MAX_C})" }
            Slider {
                class: "text-muted-foreground",
                min: 0.0,
                max: 0.4,
                step: 0.01,
                value: c() as f64,
                oninput: move |e: FormEvent| {
                    let val = e.value().parse().unwrap_or(DEFAULT_C);
                    c.set(val);
                },
            }
        }
        div { class: "w-full",
            small { class: "text-sm font-medium leading-none", "Hue (H)" }
            span { class: "ml-2 text-xs text-muted-foreground", "({h} / {MAX_H})" }
            Slider {
                class: "text-muted-foreground",
                min: 0.0,
                max: 360.0,
                step: 1.0,
                value: h() as f64,
                oninput: move |e: FormEvent| {
                    let val = e.value().parse().unwrap_or(DEFAULT_H);
                    h.set(val);
                },
            }
        }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn CopyCodeDialog(
    theme: ReadSignal<String>,
    children: Option<Element>,
    #[props(default)] trigger_variant: ButtonVariant,
    #[props(default)] trigger_size: ButtonSize,
) -> Element {
    let mut copied = use_signal(|| false);

    let handle_copy = move |_| {
        let text = theme();
        spawn(async move {
            let escaped = text.replace('`', "\\`").replace('$', "\\$");
            eval(&format!("await navigator.clipboard.writeText(`{escaped}`)")).await.ok();
            copied.set(true);
            eval("await new Promise(r => setTimeout(r, 2000))").await.ok();
            copied.set(false);
        });
    };

    let trigger_label = if let Some(c) = children {
        c
    } else {
        rsx! { "Copy code" }
    };

    rsx! {
        Dialog {
            DialogTrigger { {trigger_label} }
            DialogContent { class: "sm:max-w-[800px]",
                DialogBody {
                    DialogTitle { "Theme" }
                    p { class: "text-sm text-muted-foreground",
                        "Copy and paste the following code into your CSS file."
                    }
                    div { class: "flex flex-col gap-3",
                        div { class: "overflow-hidden relative rounded-md h-[28rem]",
                            ScrollArea { class: "w-full h-full rounded-[inherit]",
                                div { class: "table p-2 min-w-full rounded-md bg-muted",
                                    pre { class: "text-sm text-muted-foreground",
                                        code { "{theme}" }
                                    }
                                }
                            }
                            Button {
                                variant: ButtonVariant::Outline,
                                // TODO. Normally, no need to add [&_svg:not([class*='size-'])]:size-4 but it does not work without...
                                class: "absolute top-4 right-4 [&_svg:not([class*='size-'])]:size-4 ",
                                onclick: handle_copy,
                                if copied() {
                                    Check {}
                                } else {
                                    Copy {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

const RADIUS_BTN: &str = "inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors hover:bg-muted hover:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 data-[state=on]:bg-accent data-[state=on]:text-accent-foreground bg-transparent h-10 px-3";

#[component]
pub fn RadiusSelector() -> Element {
    rsx! {
        div { class: "w-full",
            small { class: "text-sm font-medium leading-none", "Radius" }
            div {
                role: "group",
                dir: "ltr",
                class: "flex gap-1 justify-between items-center",
                tabindex: "0",
                style: "outline: none;",
                button {
                    class: RADIUS_BTN,
                    "data-state": "off",
                    role: "radio",
                    "aria-checked": "false",
                    "aria-label": "Toggle radius 0",
                    tabindex: "-1",
                    "0"
                }
                button {
                    class: RADIUS_BTN,
                    "data-state": "off",
                    role: "radio",
                    "aria-checked": "false",
                    "aria-label": "Toggle radius 0.3",
                    tabindex: "-1",
                    "0.3"
                }
                button {
                    class: RADIUS_BTN,
                    "data-state": "on",
                    role: "radio",
                    "aria-checked": "true",
                    "aria-label": "Toggle radius 0.5",
                    tabindex: "-1",
                    "0.5"
                }
                button {
                    class: RADIUS_BTN,
                    "data-state": "off",
                    role: "radio",
                    "aria-checked": "false",
                    "aria-label": "Toggle radius 0.75",
                    tabindex: "-1",
                    "0.75"
                }
                button {
                    class: RADIUS_BTN,
                    "data-state": "off",
                    role: "radio",
                    "aria-checked": "false",
                    "aria-label": "Toggle radius 1.0",
                    tabindex: "-1",
                    "1.0"
                }
            }
        }
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
