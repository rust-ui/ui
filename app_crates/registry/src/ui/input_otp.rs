use std::sync::atomic::{AtomicU64, Ordering};

use dioxus::prelude::*;
use icons::Minus;
use tw_merge::tw_merge;

static OTP_COUNTER: AtomicU64 = AtomicU64::new(0);

fn use_otp_id() -> String {
    use_hook(|| {
        let id = OTP_COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("otp_{id}")
    })
}

#[component]
pub fn InputOTP(
    children: Element,
    max_length: u32,
    #[props(optional)] disabled: bool,
    #[props(into, optional)] value: Option<String>,
    #[props(into, optional)] class: Option<String>,
) -> Element {
    let container_id = use_otp_id();
    let c = tw_merge!("relative flex items-center gap-2 has-[:disabled]:opacity-50", class.as_deref().unwrap_or(""));

    rsx! {
        div {
            "data-slot": "input-otp",
            "data-otp-root": "",
            id: "{container_id}",
            class: "{c}",
            {children}
            input {
                "data-otp-input": "",
                r#type: "text",
                inputmode: "numeric",
                maxlength: "{max_length}",
                disabled: disabled,
                value: value.as_deref().unwrap_or(""),
                class: "hidden",
            }
            script { src: "/components/otp.js" }
        }
    }
}

#[component]
pub fn InputOTPGroup(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!("flex items-center", class.as_deref().unwrap_or(""));
    rsx! { div { "data-slot": "input-otp-group", class: "{c}", {children} } }
}

#[component]
pub fn InputOTPSlot(
    index: u32,
    #[props(optional)] aria_invalid: bool,
    #[props(into, optional)] class: Option<String>,
) -> Element {
    let c = tw_merge!(
        "relative flex h-9 w-9 cursor-text items-center justify-center border-y border-r border-input text-sm shadow-xs transition-all outline-none first:rounded-l-md first:border-l last:rounded-r-md data-[active=true]:z-10 data-[active=true]:border-ring data-[active=true]:ring-[3px] data-[active=true]:ring-ring/50 aria-invalid:border-destructive data-[active=true]:aria-invalid:ring-destructive/20 dark:bg-input/30",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div {
            "data-slot": "input-otp-slot",
            "data-otp-slot": "",
            "data-otp-index": "{index}",
            "data-active": "false",
            "aria-invalid": if aria_invalid { "true" } else { "" },
            class: "{c}",
            span { "data-otp-char": "" }
            div {
                "data-otp-caret": "",
                class: "flex absolute inset-0 justify-center items-center pointer-events-none",
                style: "display: none",
                div { class: "w-px h-4 duration-1000 animate-caret-blink bg-foreground" }
            }
        }
    }
}

#[component]
pub fn InputOTPSeparator(#[props(into, optional)] class: Option<String>) -> Element {
    let c = tw_merge!("flex items-center justify-center text-muted-foreground", class.as_deref().unwrap_or(""));
    rsx! {
        div { "data-slot": "input-otp-separator", role: "separator", class: "{c}",
            Minus { class: "size-4" }
        }
    }
}
