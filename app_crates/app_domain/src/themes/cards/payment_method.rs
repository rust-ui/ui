use dioxus::prelude::*;
use icons::CreditCard;
use registry::ui::button::Button;
use registry::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
use registry::ui::input::Input;
use registry::ui::label::Label;
use registry::ui::select::{Select, SelectContent, SelectGroup, SelectOption, SelectTrigger, SelectValue};

#[derive(Clone, Copy, PartialEq)]
enum PaymentMethod {
    Card,
    Paypal,
    Apple,
}

const RADIO_CARD: &str = "flex flex-col justify-between items-center p-4 text-sm font-medium leading-none bg-transparent rounded-md border-2 disabled:cursor-not-allowed disabled:opacity-70 border-muted aria-checked:border-primary hover:bg-accent hover:text-accent-foreground";

#[component]
pub fn CardPaymentMethod() -> Element {
    let mut payment_method = use_signal(|| PaymentMethod::Card);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let years: Vec<String> = (2024..=2034).map(|y| y.to_string()).collect();

    rsx! {
        Card {
            CardHeader {
                CardTitle { class: "text-2xl", "Payment Method" }
                CardDescription { "Add a new payment method to your account." }
            }
            CardContent { class: "grid gap-6",
                div { class: "grid grid-cols-3 gap-4 outline-none", dir: "ltr", tabindex: "0",
                    button {
                        class: RADIO_CARD,
                        role: "radio",
                        "aria-checked": if payment_method() == PaymentMethod::Card { "true" } else { "false" },
                        onclick: move |_| payment_method.set(PaymentMethod::Card),
                        CreditCard { class: "mb-3 size-6" }
                        span { "Card" }
                    }
                    button {
                        class: RADIO_CARD,
                        role: "radio",
                        "aria-checked": if payment_method() == PaymentMethod::Paypal { "true" } else { "false" },
                        onclick: move |_| payment_method.set(PaymentMethod::Paypal),
                        LogoPaypal {}
                        span { "Paypal" }
                    }
                    button {
                        class: RADIO_CARD,
                        role: "radio",
                        "aria-checked": if payment_method() == PaymentMethod::Apple { "true" } else { "false" },
                        onclick: move |_| payment_method.set(PaymentMethod::Apple),
                        LogoApple {}
                        span { "Apple" }
                    }
                }

                div { class: "grid gap-2",
                    Label { html_for: "name", "Name" }
                    Input { id: "name", placeholder: "First Last" }
                }
                div { class: "grid gap-2",
                    Label { html_for: "city", "City" }
                    Input { id: "city" }
                }
                div { class: "grid gap-2",
                    Label { html_for: "number", "Card number" }
                    Input { id: "number" }
                }

                div { class: "grid grid-cols-3 gap-2",
                    div { class: "grid gap-2",
                        Label { html_for: "month", "Expires" }
                        Select {
                            SelectTrigger { class: "w-[85px]",
                                SelectValue { placeholder: "Month" }
                            }
                            SelectContent {
                                SelectGroup {
                                    {months.iter().map(|m| rsx! {
                                        SelectOption { key: "{m}", value: *m, "{m}" }
                                    })}
                                }
                            }
                        }
                    }
                    div { class: "grid gap-2",
                        Label { html_for: "year", "Year" }
                        Select {
                            SelectTrigger {
                                SelectValue { placeholder: "Year" }
                            }
                            SelectContent { class: "w-[100px]",
                                SelectGroup {
                                    {years.iter().map(|y| rsx! {
                                        SelectOption { key: "{y}", value: y.as_str(), "{y}" }
                                    })}
                                }
                            }
                        }
                    }
                    div { class: "grid gap-2",
                        Label { html_for: "cvc", "CVC" }
                        Input { id: "cvc", placeholder: "CVC" }
                    }
                }
            }
            CardFooter {
                Button { class: "w-full", "Continue" }
            }
        }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn LogoApple() -> Element {
    rsx! {
        svg { role: "img", view_box: "0 0 24 24", class: "mb-3 size-6",
            path {
                d: "M12.152 6.896c-.948 0-2.415-1.078-3.96-1.04-2.04.027-3.91 1.183-4.961 3.014-2.117 3.675-.546 9.103 1.519 12.09 1.013 1.454 2.208 3.09 3.792 3.039 1.52-.065 2.09-.987 3.935-.987 1.831 0 2.35.987 3.96.948 1.637-.026 2.676-1.48 3.676-2.948 1.156-1.688 1.636-3.325 1.662-3.415-.039-.013-3.182-1.221-3.22-4.857-.026-3.04 2.48-4.494 2.597-4.559-1.429-2.09-3.623-2.324-4.39-2.376-2-.156-3.675 1.09-4.61 1.09zM15.53 3.83c.843-1.012 1.4-2.427 1.245-3.83-1.207.052-2.662.805-3.532 1.818-.78.896-1.454 2.338-1.273 3.714 1.338.104 2.715-.688 3.559-1.701",
                fill: "currentColor",
            }
        }
    }
}

#[component]
pub fn LogoPaypal() -> Element {
    rsx! {
        svg { role: "img", view_box: "0 0 24 24", class: "mb-3 size-6",
            path {
                d: "M7.076 21.337H2.47a.641.641 0 0 1-.633-.74L4.944.901C5.026.382 5.474 0 5.998 0h7.46c2.57 0 4.578.543 5.69 1.81 1.01 1.15 1.304 2.42 1.012 4.287-.023.143-.047.288-.077.437-.983 5.05-4.349 6.797-8.647 6.797h-2.19c-.524 0-.968.382-1.05.9l-1.12 7.106zm14.146-14.42a3.35 3.35 0 0 0-.607-.541c-.013.076-.026.175-.041.254-.93 4.778-4.005 7.201-9.138 7.201h-2.19a.563.563 0 0 0-.556.479l-1.187 7.527h-.506l-.24 1.516a.56.56 0 0 0 .554.647h3.882c.46 0 .85-.334.922-.788.06-.26.76-4.852.816-5.09a.932.932 0 0 1 .923-.788h.58c3.76 0 6.705-1.528 7.565-5.946.36-1.847.174-3.388-.777-4.471z",
                fill: "currentColor",
            }
        }
    }
}
