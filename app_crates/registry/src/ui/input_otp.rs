use icons::Minus;
use leptos::prelude::*;
use tw_merge::*;

use crate::hooks::use_random::use_random_id;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn InputOTP(
    children: Children,
    max_length: u32,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] value: String,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let id = use_random_id();
    let container_id = format!("otp_{}", id);
    let class = tw_merge!("relative flex items-center gap-2 has-[:disabled]:opacity-50", class);

    view! {
        <div data-slot="input-otp" data-otp-root="" id=container_id class=class>
            {children()}
            <input
                data-otp-input=""
                type="text"
                inputmode="numeric"
                maxlength=max_length.to_string()
                disabled=disabled
                prop:value=value
                class="sr-only"
            />
            <script src="/app/otp.js" />
        </div>
    }
}

#[component]
pub fn InputOTPGroup(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let class = tw_merge!("flex items-center", class);
    view! {
        <div data-slot="input-otp-group" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn InputOTPSlot(
    index: u32,
    #[prop(optional)] aria_invalid: bool,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let class = tw_merge!(
        "relative flex h-9 w-9 cursor-text items-center justify-center border-y border-r border-input text-sm shadow-xs transition-all outline-none first:rounded-l-md first:border-l last:rounded-r-md data-[active=true]:z-10 data-[active=true]:border-ring data-[active=true]:ring-[3px] data-[active=true]:ring-ring/50 aria-invalid:border-destructive data-[active=true]:aria-invalid:ring-destructive/20 dark:bg-input/30",
        class
    );

    view! {
        <div
            data-slot="input-otp-slot"
            data-otp-slot=""
            data-otp-index=index.to_string()
            data-active="false"
            class=class
            attr:aria-invalid=aria_invalid.then_some("true")
        >
            <span data-otp-char=""></span>
            <div
                data-otp-caret=""
                class="flex absolute inset-0 justify-center items-center pointer-events-none"
                style="display: none"
            >
                <div class="w-px h-4 duration-1000 animate-caret-blink bg-foreground"></div>
            </div>
        </div>
    }
}

#[component]
pub fn InputOTPSeparator(#[prop(optional, into)] class: String) -> impl IntoView {
    let class = tw_merge!("flex items-center justify-center text-muted-foreground", class);
    view! {
        <div data-slot="input-otp-separator" role="separator" class=class>
            <Minus class="size-4" />
        </div>
    }
}
