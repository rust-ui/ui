use dioxus::prelude::*;
use icons::Check;
use tw_merge::tw_merge;

#[component]
pub fn Checkbox(
    #[props(into, default)] class: Option<String>,
    #[props(default = false)] checked: bool,
    #[props(default = false)] disabled: bool,
    #[props(default, optional)] on_checked_change: Option<EventHandler<bool>>,
    #[props(default, optional)] on_change: Option<EventHandler<bool>>,
    #[props(default = "Checkbox".to_string(), into)] aria_label: String,
) -> Element {
    let checked_state = if checked { "checked" } else { "unchecked" };
    let merged = tw_merge!(
        "peer border-input dark:bg-input/30 data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground dark:data-[state=checked]:bg-primary data-[state=checked]:border-primary focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive size-4 shrink-0 rounded-[4px] border shadow-xs transition-shadow outline-none focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        button {
            "data-name": "Checkbox",
            class: "{merged}",
            "data-state": checked_state,
            r#type: "button",
            role: "checkbox",
            "aria-checked": "{checked}",
            "aria-label": "{aria_label}",
            disabled,
            onclick: move |_| {
                if !disabled {
                    if let Some(handler) = &on_checked_change {
                        handler.call(!checked);
                    }
                    if let Some(handler) = &on_change {
                        handler.call(!checked);
                    }
                }
            },
            span {
                "data-name": "CheckboxIndicator",
                class: "flex justify-center items-center text-current transition-none",
                if checked {
                    Check { class: "size-3.5" }
                }
            }
        }
    }
}
