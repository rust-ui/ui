use icons::CreditCard;
use leptos::prelude::*;
use leptos_ui::clx;
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

#[component]
pub fn CardPaymentMethod() -> impl IntoView {
    clx! {RadioGroup, div, "grid grid-cols-3 gap-4 outline-none"}
    clx! {RadioCard, button, "flex flex-col justify-between items-center p-4 text-sm font-medium leading-none bg-transparent rounded-md border-2 disabled:cursor-not-allowed disabled:opacity-70 border-muted aria-checked:border-primary hover:bg-accent hover:text-accent-foreground"}

    let payment_method_signal = RwSignal::new(PaymentMethod::Card);
    let month_signal = RwSignal::new(None::<String>);
    let year_signal = RwSignal::new(None::<String>);

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

    let years: Vec<String> = (2024..=2034).map(|year| year.to_string()).collect();

    view! {
        <Card>
            <CardHeader>
                <CardTitle class="text-2xl">Payment Method</CardTitle>
                <CardDescription>Add a new payment method to your account.</CardDescription>
            </CardHeader>
            <CardContent class="grid gap-6">
                <RadioGroup attr:dir="ltr" attr:tabindex="0">
                    <RadioCard
                        attr:role="radio"
                        attr:aria-checked=move || { (payment_method_signal.get() == PaymentMethod::Card).to_string() }
                        on:click=move |_| payment_method_signal.set(PaymentMethod::Card)
                    >
                        <CreditCard class="mb-3 size-6" />
                        <span>Card</span>
                    </RadioCard>
                    <RadioCard
                        attr:role="radio"
                        attr:aria-checked=move || { (payment_method_signal.get() == PaymentMethod::Paypal).to_string() }
                        on:click=move |_| payment_method_signal.set(PaymentMethod::Paypal)
                    >
                        <LogoPaypal />
                        <span>Paypal</span>
                    </RadioCard>
                    <RadioCard
                        attr:role="radio"
                        attr:aria-checked=move || { (payment_method_signal.get() == PaymentMethod::Apple).to_string() }
                        on:click=move |_| payment_method_signal.set(PaymentMethod::Apple)
                    >
                        <LogoApple />
                        <span>Apple</span>
                    </RadioCard>
                </RadioGroup>

                <div class="grid gap-2">
                    <Label html_for="name">Name</Label>
                    <Input attr:id="name" attr:placeholder="First Last" />
                </div>
                <div class="grid gap-2">
                    <Label html_for="city">City</Label>
                    <Input attr:id="city" attr:placeholder="" />
                </div>
                <div class="grid gap-2">
                    <Label html_for="number">Card number</Label>
                    <Input attr:id="number" attr:placeholder="" />
                </div>

                <div class="grid grid-cols-3 gap-2">
                    <div class="grid gap-2">
                        <Label html_for="month">Expires</Label>
                        <Select>
                            <SelectTrigger id="month" class="w-[85px]">
                                <SelectValue placeholder="Month" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectGroup>
                                    {months
                                        .into_iter()
                                        .map(|month| {
                                            let month_str = month.to_string();
                                            let month_str_clone = month_str.clone();
                                            view! {
                                                <SelectOption
                                                    aria_selected=Signal::derive(move || {
                                                        month_signal.get().as_deref() == Some(month)
                                                    })
                                                    value=month_str.clone()
                                                    on:click=move |_| {
                                                        month_signal.set(Some(month_str_clone.clone()))
                                                    }
                                                >
                                                    {month_str}
                                                </SelectOption>
                                            }
                                        })
                                        .collect_view()}
                                </SelectGroup>
                            </SelectContent>
                        </Select>
                    </div>
                    <div class="grid gap-2">
                        <Label html_for="year">Year</Label>
                        <Select>
                            <SelectTrigger id="year">
                                <SelectValue placeholder="Year" />
                            </SelectTrigger>
                            <SelectContent class="w-[100px]">
                                <SelectGroup>
                                    {years
                                        .into_iter()
                                        .map(|year| {
                                            let year_str = year.clone();
                                            let year_str_clone = year_str.clone();
                                            view! {
                                                <SelectOption
                                                    aria_selected=Signal::derive(move || {
                                                        year_signal.get().as_deref() == Some(year.as_str())
                                                    })
                                                    value=year_str.clone()
                                                    on:click=move |_| year_signal.set(Some(year_str_clone.clone()))
                                                >
                                                    {year_str}
                                                </SelectOption>
                                            }
                                        })
                                        .collect_view()}
                                </SelectGroup>
                            </SelectContent>
                        </Select>
                    </div>
                    <div class="grid gap-2">
                        <Label html_for="cvc">CVC</Label>
                        <Input attr:id="cvc" attr:placeholder="CVC" />
                    </div>
                </div>
            </CardContent>
            <CardFooter>
                <Button class="w-full">Continue</Button>
            </CardFooter>
        </Card>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn LogoApple() -> impl IntoView {
    view! {
        <svg role="img" viewBox="0 0 24 24" class="mb-3 size-6">
            <path
                d="M12.152 6.896c-.948 0-2.415-1.078-3.96-1.04-2.04.027-3.91 1.183-4.961 3.014-2.117 3.675-.546 9.103 1.519 12.09 1.013 1.454 2.208 3.09 3.792 3.039 1.52-.065 2.09-.987 3.935-.987 1.831 0 2.35.987 3.96.948 1.637-.026 2.676-1.48 3.676-2.948 1.156-1.688 1.636-3.325 1.662-3.415-.039-.013-3.182-1.221-3.22-4.857-.026-3.04 2.48-4.494 2.597-4.559-1.429-2.09-3.623-2.324-4.39-2.376-2-.156-3.675 1.09-4.61 1.09zM15.53 3.83c.843-1.012 1.4-2.427 1.245-3.83-1.207.052-2.662.805-3.532 1.818-.78.896-1.454 2.338-1.273 3.714 1.338.104 2.715-.688 3.559-1.701"
                fill="currentColor"
            ></path>
        </svg>
    }
}

#[component]
pub fn LogoPaypal() -> impl IntoView {
    view! {
        <svg role="img" viewBox="0 0 24 24" class="mb-3 size-6">
            <path
                d="M7.076 21.337H2.47a.641.641 0 0 1-.633-.74L4.944.901C5.026.382 5.474 0 5.998 0h7.46c2.57 0 4.578.543 5.69 1.81 1.01 1.15 1.304 2.42 1.012 4.287-.023.143-.047.288-.077.437-.983 5.05-4.349 6.797-8.647 6.797h-2.19c-.524 0-.968.382-1.05.9l-1.12 7.106zm14.146-14.42a3.35 3.35 0 0 0-.607-.541c-.013.076-.026.175-.041.254-.93 4.778-4.005 7.201-9.138 7.201h-2.19a.563.563 0 0 0-.556.479l-1.187 7.527h-.506l-.24 1.516a.56.56 0 0 0 .554.647h3.882c.46 0 .85-.334.922-.788.06-.26.76-4.852.816-5.09a.932.932 0 0 1 .923-.788h.58c3.76 0 6.705-1.528 7.565-5.946.36-1.847.174-3.388-.777-4.471z"
                fill="currentColor"
            ></path>
        </svg>
    }
}
