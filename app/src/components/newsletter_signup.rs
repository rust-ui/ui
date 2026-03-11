use icons::Send;
use leptos::prelude::*;
use leptos::server_fn::codec::{GetUrl, Json};
use registry::ui::button::Button;
use registry::ui::form::FormError;
use registry::ui::input_group::{InputGroup, InputGroupAddon, InputGroupInput};
#[cfg(feature = "ssr")]
use validator::Validate;

#[derive(Clone, Debug, PartialEq)]
enum SubmitStatus {
    Idle,
    Submitting,
    Success,
    Error(String),
}

#[component]
pub fn NewsletterSignup() -> impl IntoView {
    let email_signal = RwSignal::new(String::new());
    let status_signal = RwSignal::new(SubmitStatus::Idle);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let email = email_signal.get();
        if email.is_empty() {
            status_signal.set(SubmitStatus::Error("Please enter an email".to_string()));
            return;
        }

        status_signal.set(SubmitStatus::Submitting);

        leptos::task::spawn_local(async move {
            match subscribe_newsletter(email).await {
                Ok(_) => {
                    status_signal.set(SubmitStatus::Success);
                    email_signal.set(String::new());
                }
                Err(err) => {
                    use leptos::prelude::ServerFnError;
                    let error_msg = match err {
                        ServerFnError::ServerError(msg) => msg,
                        _ => err.to_string(),
                    };
                    status_signal.set(SubmitStatus::Error(error_msg));
                }
            }
        });
    };

    let button_text = move || match status_signal.get() {
        SubmitStatus::Idle => "Subscribe",
        SubmitStatus::Submitting => "Subscribing...",
        SubmitStatus::Success => "Subscribed!",
        SubmitStatus::Error(_) => "Subscribe",
    };

    let is_disabled = move || {
        matches!(status_signal.get(), SubmitStatus::Submitting | SubmitStatus::Success | SubmitStatus::Error(_))
    };

    view! {
        <div class="overflow-hidden relative py-14 px-4 rounded-xl sm:px-8 @container bg-zinc-900 dark">
            <DecorativeGlowSvg filter_id="_r_55_a" class="absolute top-0 left-0 -translate-x-1/2 pointer-events-none" />
            <DecorativeGlowSvg
                filter_id="_r_56_a"
                class="absolute right-0 bottom-0 translate-x-1/4 pointer-events-none"
            />
            <div class="flex flex-col gap-6 justify-between items-center @2xl:flex-row">
                <h2 class="text-center font-heading text-pretty text-2xl/[1.1] text-foreground @2xl:text-left md:text-3xl/[1.1]">
                    Get notified when new stuff drops.
                </h2>
                <form class="space-y-4" on:submit=on_submit>
                    <div class="space-y-2">
                        <div class="inline-flex gap-2">
                            <InputGroup class="h-10 rounded-full border-zinc-600/65 bg-zinc-700/30 md:min-w-64">
                                <InputGroupAddon>
                                    <Send />
                                </InputGroupAddon>
                                <InputGroupInput
                                    class="text-zinc-100 placeholder:text-zinc-500 [&:-webkit-autofill]:bg-zinc-700/30 [&:-webkit-autofill]:[-webkit-text-fill-color:#fff] [&:-webkit-autofill]:[transition:background-color_5000000s_ease-in-out_0s]"
                                    attr:placeholder="Enter your email..."
                                    attr:aria-label="Subscribe to the newsletter"
                                    attr:required=true
                                    attr:r#type="email"
                                    attr:data-bwignore="true"
                                    attr:data-1p-ignore="true"
                                    attr:data-lpignore="true"
                                    attr:autocomplete="off"
                                    attr:disabled=is_disabled
                                    prop:value=email_signal
                                    on:input=move |ev| {
                                        email_signal.set(event_target_value(&ev));
                                    }
                                />
                            </InputGroup>
                            <Button class="h-10 rounded-full" attr:r#type="submit" attr:disabled=is_disabled>
                                {button_text}
                            </Button>
                        </div>
                        {move || match status_signal.get() {
                            SubmitStatus::Error(msg) => view! { <FormError>{msg}</FormError> }.into_any(),
                            _ => ().into_any(),
                        }}
                    </div>
                </form>
            </div>
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[cfg(feature = "ssr")]
#[derive(Clone, Debug, Validate)]
struct EmailInput {
    #[validate(email)]
    email: String,
}

/// * We need to specify input & output for mobile targets (Android, iOS) to work. DO NOT CHANGE.
#[server(SubscribeNewsletter, "/api", input = GetUrl, output = Json)]
async fn subscribe_newsletter(email: String) -> Result<String, ServerFnError> {
    use std::env;

    use resend_rs::Resend;
    use resend_rs::types::CreateContactOptions;

    // Validate email format
    let email_input = EmailInput { email: email.clone() };
    if let Err(err) = email_input.validate() {
        tracing::error!("Email validation failed: {}", err);
        return Err(ServerFnError::ServerError(format!("Invalid email format: {err}")));
    }

    let Ok(token) = env::var("RESEND_TOKEN") else {
        return Err(ServerFnError::ServerError("RESEND_TOKEN must be set".to_string()));
    };
    let Ok(audience_id) = env::var("RESEND_AUDIENCE_ID") else {
        return Err(ServerFnError::ServerError("RESEND_AUDIENCE_ID must be set".to_string()));
    };

    // Check if contact already exists using direct HTTP call
    let client = reqwest::Client::new();
    let url = format!("https://api.resend.com/audiences/{}/contacts/{}", audience_id, email);

    match client.get(&url).header("Authorization", format!("Bearer {}", token)).send().await {
        Ok(response) => {
            if response.status().is_success() {
                tracing::error!("Email already subscribed: {}", email);
                return Err(ServerFnError::ServerError("This email is already subscribed!".to_string()));
            }
        }
        Err(err) => {
            tracing::error!("Failed to check contact existence: {}", err);
        }
    }

    // Create contact using the API for resend-rs 0.19.0
    let resend = Resend::new(&token);

    let contact = CreateContactOptions::new(&email).with_audience_id(&audience_id).with_unsubscribed(false);

    // Add contact to audience
    match resend.contacts.create(contact).await {
        Ok(_) => Ok("Successfully subscribed!".to_string()),
        Err(err) => {
            tracing::error!("Failed to subscribe email: {} - Error: {}", email, err);
            Err(ServerFnError::ServerError(format!("Failed to subscribe: {err}")))
        }
    }
}

#[component]
fn DecorativeGlowSvg(#[prop(into)] filter_id: String, #[prop(into)] class: String) -> impl IntoView {
    let filter_url = format!("url(#{})", filter_id);

    view! {
        <svg class=class xmlns="http://www.w3.org/2000/svg" width="267" height="268" fill="none" aria-hidden="true">
            <g filter=filter_url style="mix-blend-mode: plus-lighter;">
                <path
                    fill="#fff"
                    fill-opacity=".48"
                    d="M189 76.284 242.642 24 189 83.753v19.691l-8.148-6.11L24 244 176.099 89.864v-13.58H189Z"
                ></path>
            </g>
            <defs>
                <filter
                    id=filter_id
                    width="266.642"
                    height="268"
                    x="0"
                    y="0"
                    color-interpolation-filters="sRGB"
                    filterUnits="userSpaceOnUse"
                >
                    <feFlood flood-opacity="0" result="BackgroundImageFix"></feFlood>
                    <feBlend in="SourceGraphic" in2="BackgroundImageFix" result="shape"></feBlend>
                    <feGaussianBlur result="effect1_foregroundBlur_809_24" stdDeviation="12"></feGaussianBlur>
                </filter>
            </defs>
        </svg>
    }
}
