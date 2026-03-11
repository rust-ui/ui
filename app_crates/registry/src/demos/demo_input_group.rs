use icons::{Check, CreditCard, Info, Mail, Search, Star};
use leptos::prelude::*;

use crate::ui::input_group::{InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupInput};

#[component]
pub fn DemoInputGroup() -> impl IntoView {
    view! {
        <div class="grid gap-6 w-full max-w-sm">
            <InputGroup>
                <InputGroupInput placeholder="Search..." />
                <InputGroupAddon>
                    <Search />
                </InputGroupAddon>
            </InputGroup>

            <InputGroup>
                <InputGroupInput placeholder="Enter your email" />
                <InputGroupAddon>
                    <Mail />
                </InputGroupAddon>
            </InputGroup>

            <InputGroup>
                <InputGroupInput placeholder="Card number" />
                <InputGroupAddon>
                    <CreditCard />
                </InputGroupAddon>
                <InputGroupAddon align=InputGroupAddonAlign::InlineEnd>
                    <Check />
                </InputGroupAddon>
            </InputGroup>

            <InputGroup>
                <InputGroupInput placeholder="Card number" />
                <InputGroupAddon align=InputGroupAddonAlign::InlineEnd>
                    <Star />
                    <Info />
                </InputGroupAddon>
            </InputGroup>
        </div>
    }
}
