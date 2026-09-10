use dioxus::prelude::*;
use tw_merge::tw_merge;

#[derive(Default, Clone, PartialEq)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
    Mobile,
}

#[derive(Default, Clone, PartialEq)]
pub enum ButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
    Icon,
}

impl ButtonVariant {
    fn class(&self) -> &'static str {
        match self {
            ButtonVariant::Default => "bg-primary text-primary-foreground shadow hover:bg-primary/90",
            ButtonVariant::Destructive => "bg-destructive text-white shadow-sm hover:bg-destructive/90",
            ButtonVariant::Outline => {
                "border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground"
            }
            ButtonVariant::Secondary => "bg-secondary text-secondary-foreground shadow-sm hover:bg-secondary/80",
            ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
            ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
            ButtonVariant::Mobile => "bg-primary text-primary-foreground px-6 py-3 rounded-[24px]",
        }
    }
}

impl ButtonSize {
    fn class(&self) -> &'static str {
        match self {
            ButtonSize::Default => "h-9 px-4 py-2 has-[>svg]:px-3",
            ButtonSize::Sm => "h-8 rounded-md gap-1.5 px-3 has-[>svg]:px-2.5",
            ButtonSize::Lg => "h-10 rounded-md px-6 has-[>svg]:px-4",
            ButtonSize::Icon => "size-9",
        }
    }
}

#[component]
pub fn Button(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(into, optional)] class: Option<String>,
    #[props(optional)] onclick: Option<EventHandler<MouseEvent>>,
    #[props(default = false)] disabled: bool,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors",
        "focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring",
        "disabled:pointer-events-none disabled:opacity-50 cursor-pointer",
        variant.class(),
        size.class(),
        class.as_deref().unwrap_or("")
    );

    rsx! {
        button {
            class: "{merged}",
            disabled,
            onclick: move |e| {
                if let Some(handler) = &onclick {
                    handler.call(e);
                }
            },
            {children}
        }
    }
}
