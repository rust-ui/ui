use dioxus::prelude::*;
use icons::Send;
use registry::ui::button::Button;
use registry::ui::input::InputType;
use registry::ui::input_group::{InputGroup, InputGroupAddon, InputGroupInput};

#[derive(Clone, Debug, PartialEq)]
enum SubmitStatus {
    Idle,
    Submitting,
    Success,
    Error(String),
}

#[component]
pub fn NewsletterSignup() -> Element {
    let mut email = use_signal(String::new);
    let mut status = use_signal(|| SubmitStatus::Idle);

    let on_submit = move |ev: FormEvent| {
        ev.prevent_default();
        let addr = email.read().clone();
        if addr.is_empty() {
            status.set(SubmitStatus::Error("Please enter an email".to_string()));
            return;
        }
        status.set(SubmitStatus::Submitting);
        spawn(async move {
            match subscribe_newsletter(addr).await {
                Ok(_) => {
                    status.set(SubmitStatus::Success);
                    email.set(String::new());
                }
                Err(e) => {
                    status.set(SubmitStatus::Error(e.to_string()));
                }
            }
        });
    };

    let button_label = match *status.read() {
        SubmitStatus::Idle => "Subscribe",
        SubmitStatus::Submitting => "Subscribing...",
        SubmitStatus::Success => "Subscribed!",
        SubmitStatus::Error(_) => "Subscribe",
    };

    let disabled = matches!(*status.read(), SubmitStatus::Submitting | SubmitStatus::Success);

    let error_msg = match status.read().clone() {
        SubmitStatus::Error(msg) => Some(msg),
        _ => None,
    };

    rsx! {
        div { class: "overflow-hidden relative py-14 px-4 rounded-xl sm:px-8 @container bg-zinc-900 dark",
            DecorativeGlowSvg {
                filter_id: "_r_55_a",
                class: "absolute top-0 left-0 -translate-x-1/2 pointer-events-none",
            }
            DecorativeGlowSvg {
                filter_id: "_r_56_a",
                class: "absolute right-0 bottom-0 translate-x-1/4 pointer-events-none",
            }
            div { class: "flex flex-col gap-6 justify-between items-center @2xl:flex-row",
                h2 { class: "text-center font-heading text-pretty text-2xl/[1.1] text-foreground @2xl:text-left md:text-3xl/[1.1]",
                    "Get notified when new stuff drops."
                }
                form { class: "space-y-4", onsubmit: on_submit,
                    div { class: "space-y-2",
                        div { class: "inline-flex gap-2",
                            InputGroup { class: "h-10 rounded-full border-zinc-600/65 bg-zinc-700/30 md:min-w-64",
                                InputGroupAddon { Send { class: "text-zinc-400 size-4" } }
                                InputGroupInput {
                                    class: "text-zinc-100 placeholder:text-zinc-500 [&:-webkit-autofill]:bg-zinc-700/30 [&:-webkit-autofill]:[-webkit-text-fill-color:#fff] [&:-webkit-autofill]:[transition:background-color_5000000s_ease-in-out_0s]",
                                    r#type: InputType::Email,
                                    placeholder: "Enter your email...",
                                    autocomplete: "off",
                                    required: true,
                                    disabled,
                                    value: email.read().clone(),
                                    oninput: move |ev: FormEvent| email.set(ev.value()),
                                }
                            }
                            Button { class: "h-10 rounded-full", disabled, button_type: "submit",
                                "{button_label}"
                            }
                        }
                        if let Some(msg) = error_msg {
                            p { class: "text-sm text-destructive", "{msg}" }
                        }
                    }
                }
            }
        }
    }
}

#[server]
async fn subscribe_newsletter(email: String) -> Result<String, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use std::env;

        use resend_rs::Resend;
        use resend_rs::types::CreateContactOptions;
        use validator::Validate;

        #[derive(validator::Validate)]
        struct EmailInput {
            #[validate(email)]
            email: String,
        }

        let input = EmailInput { email: email.clone() };
        if let Err(err) = input.validate() {
            tracing::error!("Email validation failed: {}", err);
            return Err(ServerFnError::new(format!("Invalid email format: {err}")));
        }

        let Ok(token) = env::var("RESEND_TOKEN") else {
            return Err(ServerFnError::new("RESEND_TOKEN must be set".to_string()));
        };
        let Ok(audience_id) = env::var("RESEND_AUDIENCE_ID") else {
            return Err(ServerFnError::new("RESEND_AUDIENCE_ID must be set".to_string()));
        };

        let client = reqwest::Client::new();
        let url = format!("https://api.resend.com/audiences/{}/contacts/{}", audience_id, email);
        match client.get(&url).header("Authorization", format!("Bearer {token}")).send().await {
            Ok(resp) if resp.status().is_success() => {
                tracing::error!("Email already subscribed: {}", email);
                return Err(ServerFnError::new("This email is already subscribed!".to_string()));
            }
            Err(err) => {
                tracing::error!("Failed to check contact existence: {}", err);
            }
            _ => {}
        }

        let resend = Resend::new(&token);
        let contact = CreateContactOptions::new(&email).with_audience_id(&audience_id).with_unsubscribed(false);
        match resend.contacts.create(contact).await {
            Ok(_) => Ok("Successfully subscribed!".to_string()),
            Err(err) => {
                tracing::error!("Failed to subscribe email: {} - Error: {}", email, err);
                Err(ServerFnError::new(format!("Failed to subscribe: {err}")))
            }
        }
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = email;
        Err(ServerFnError::new("Server not available".to_string()))
    }
}

#[component]
fn DecorativeGlowSvg(filter_id: String, class: String) -> Element {
    let filter_url = format!("url(#{})", filter_id);
    rsx! {
        svg {
            class: "{class}",
            xmlns: "http://www.w3.org/2000/svg",
            width: "267",
            height: "268",
            fill: "none",
            "aria-hidden": "true",
            g { filter: "{filter_url}", style: "mix-blend-mode: plus-lighter;",
                path {
                    fill: "#fff",
                    fill_opacity: ".48",
                    d: "M189 76.284 242.642 24 189 83.753v19.691l-8.148-6.11L24 244 176.099 89.864v-13.58H189Z",
                }
            }
            defs {
                filter {
                    id: "{filter_id}",
                    width: "266.642",
                    height: "268",
                    x: "0",
                    y: "0",
                    "color-interpolation-filters": "sRGB",
                    filterUnits: "userSpaceOnUse",
                    feFlood { flood_opacity: "0", result: "BackgroundImageFix" }
                    feBlend { "in": "SourceGraphic", in2: "BackgroundImageFix", result: "shape" }
                    feGaussianBlur { result: "effect1_foregroundBlur_809_24", std_deviation: "12" }
                }
            }
        }
    }
}
