use dioxus::prelude::*;
use tw_merge::tw_merge;

#[derive(Default, Clone, PartialEq)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    Warning,
    Success,
    Outline,
    Secondary,
    Ghost,
    Link,
}

#[allow(dead_code)]
#[derive(Default, Clone, PartialEq)]
pub enum ButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
    Icon,
    IconSm,
    IconXs,
}

impl ButtonVariant {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            ButtonVariant::Default => "bg-primary text-primary-foreground shadow-xs hover:bg-primary/90",
            ButtonVariant::Destructive => {
                "bg-destructive text-white shadow-xs hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60"
            }
            ButtonVariant::Warning => "bg-warning text-warning-foreground hover:bg-warning/90",
            ButtonVariant::Success => "bg-success text-success-foreground hover:bg-success/90",
            ButtonVariant::Outline => {
                "border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/5"
            }
            ButtonVariant::Secondary => "bg-secondary text-secondary-foreground shadow-xs hover:bg-secondary/80",
            ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50",
            ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
        }
    }
}

impl ButtonSize {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            ButtonSize::Default => "h-9 px-4 py-2 has-[>svg]:px-3",
            ButtonSize::Sm => "h-8 rounded-md gap-1.5 px-3 has-[>svg]:px-2.5",
            ButtonSize::Lg => "h-10 rounded-md px-6 has-[>svg]:px-4",
            ButtonSize::Icon => "size-9",
            ButtonSize::IconSm => "size-8 rounded-md",
            ButtonSize::IconXs => "size-6 rounded-md",
        }
    }
}

#[component]
pub fn Button(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] id: Option<String>,
    #[props(default = ButtonVariant::default())] variant: ButtonVariant,
    #[props(default = ButtonSize::default())] size: ButtonSize,
    #[props(optional)] disabled: bool,
    #[props(into, optional)] href: Option<String>,
    // TODO: replace button_type with a proper ButtonType enum (e.g. ButtonType::Submit/Reset/Button)
    #[props(into, optional)] button_type: Option<String>,
    #[props(optional)] onclick: Option<EventHandler<MouseEvent>>,
    #[props(optional)] onpointerdown: Option<EventHandler<PointerEvent>>,
    #[props(optional)] onpointerup: Option<EventHandler<PointerEvent>>,
    #[props(optional)] onpointerleave: Option<EventHandler<PointerEvent>>,
    #[props(optional)] onpointercancel: Option<EventHandler<PointerEvent>>,
    #[props(into, optional)] aria_label: Option<String>,
    #[props(into, optional)] title: Option<String>,
    children: Element,
) -> Element {
    let merged_class = tw_merge!(
        "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive w-fit hover:cursor-pointer active:scale-[0.98] active:opacity-100 touch-manipulation [-webkit-tap-highlight-color:transparent] select-none [-webkit-touch-callout:none]",
        variant.as_str(),
        size.as_str(),
        class.as_deref().unwrap_or("")
    );

    if let Some(url) = href {
        rsx! {
            Link {
                id: id,
                class: "{merged_class}",
                to: url,
                "aria-label": aria_label.as_deref(),
                title: title.as_deref(),
                {children}
            }
        }
    } else {
        rsx! {
            button {
                id: id.as_deref(),
                class: "{merged_class}",
                r#type: button_type.as_deref().unwrap_or("button"),
                disabled,
                "aria-label": aria_label.as_deref(),
                title: title.as_deref(),
                onclick: move |e| {
                    if let Some(handler) = &onclick {
                        handler.call(e);
                    }
                },
                onpointerdown: move |e| {
                    if let Some(handler) = &onpointerdown {
                        handler.call(e);
                    }
                },
                onpointerup: move |e| {
                    if let Some(handler) = &onpointerup {
                        handler.call(e);
                    }
                },
                onpointerleave: move |e| {
                    if let Some(handler) = &onpointerleave {
                        handler.call(e);
                    }
                },
                onpointercancel: move |e| {
                    if let Some(handler) = &onpointercancel {
                        handler.call(e);
                    }
                },
                {children}
            }
        }
    }
}
