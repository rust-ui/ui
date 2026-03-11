---
title: "Input Phone"
name: "input_phone"
cargo_dependencies: []
registry_dependencies: ["command", "input", "popover"]
type: "components:ui"
path: "ui/input_phone.rs"
---

# Input Phone

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add input_phone
```

## Component Code

```rust
use icons::{ChevronsUpDown, Search, X};
use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::*;

use crate::components::ui::command::{
    Command, CommandEmpty, CommandGroup, CommandGroupLabel, CommandInput, CommandItem, CommandList,
};
use crate::components::ui::input::Input;
use crate::components::ui::popover::{Popover, PopoverAlign, PopoverContent, PopoverTrigger};
use crate::utils::country::Country;
use crate::utils::phone_number::{PhoneFormat, PhoneNumber};

const COMMON_COUNTRIES: &[Country] = &[
    Country::UnitedStatesOfAmerica,
    Country::UnitedKingdom,
    Country::France,
    Country::Germany,
    Country::Canada,
    Country::Australia,
    Country::Spain,
    Country::Italy,
    Country::Japan,
    Country::China,
    Country::India,
    Country::Brazil,
    Country::Mexico,
];

mod components {
    use super::*;
    clx! {InputPhoneWrapper, div, "flex w-full"}
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn CountryItem(country: Country, selected_country: RwSignal<Country>) -> impl IntoView {
    let search_value = format!("{} {} {}", country.name(), country.alpha2(), country.dial_code_formatted(),);
    let is_selected = Signal::derive(move || selected_country.get() == country);

    view! {
        <CommandItem
            value=search_value
            selected=is_selected
            reserve_check_space=true
            on_select=Callback::new(move |_| {
                selected_country.set(country);
            })
        >
            <span class="text-base">{country.flag_emoji()}</span>
            <span class="flex-1 truncate">{country.name()}</span>
            <span class="w-12 text-right text-muted-foreground">{country.dial_code_formatted()}</span>
        </CommandItem>
    }
}

#[component]
pub fn InputPhone(
    #[prop(optional, into)] class: String,
    #[prop(optional)] value_signal: Option<RwSignal<PhoneNumber>>,
    #[prop(optional)] country_signal: Option<RwSignal<Country>>,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] invalid: MaybeProp<bool>,
    #[prop(optional, into)] on_blur: Option<Callback<()>>,
) -> impl IntoView {
    let internal_value_signal = RwSignal::new(PhoneNumber::default());
    let internal_country_signal = RwSignal::new(Country::UnitedStatesOfAmerica);

    let value = value_signal.unwrap_or(internal_value_signal);
    let selected_country = country_signal.unwrap_or(internal_country_signal);

    let wrapper_class = tw_merge!("flex w-full", class);

    view! {
        <InputPhoneWrapper class=wrapper_class>
            <Popover align=PopoverAlign::Start>
                <PopoverTrigger
                    class="gap-1 px-3 w-auto rounded-r-none border-r-0"
                    attr:disabled=disabled
                    attr:aria-label="Select country"
                >
                    <span class="text-base">{move || selected_country.get().flag_emoji()}</span>
                    <span class="text-xs text-muted-foreground">
                        {move || selected_country.get().dial_code_formatted()}
                    </span>
                    <ChevronsUpDown class="ml-1 opacity-50 size-3" />
                </PopoverTrigger>

                <PopoverContent class="p-0 w-[280px]">
                    <Command>
                        <div class="flex gap-2 items-center px-2 border-b">
                            <Search class="size-4 text-muted-foreground shrink-0" />
                            <CommandInput attr:placeholder="Search country..." />
                        </div>
                        <CommandList class="min-h-0 max-h-[280px]">
                            <CommandEmpty>"No country found."</CommandEmpty>

                            // Common countries
                            <CommandGroup>
                                {COMMON_COUNTRIES
                                    .iter()
                                    .map(|&country| {
                                        view! { <CountryItem country selected_country /> }
                                    })
                                    .collect_view()}
                            </CommandGroup>

                            // Separator + rest of countries
                            <CommandGroup>
                                <CommandGroupLabel>"All countries"</CommandGroupLabel>
                                {Country::all()
                                    .iter()
                                    .filter(|c| !COMMON_COUNTRIES.contains(c))
                                    .map(|&country| {
                                        view! { <CountryItem country selected_country /> }
                                    })
                                    .collect_view()}
                            </CommandGroup>
                        </CommandList>
                    </Command>
                </PopoverContent>
            </Popover>

            // Phone number input - displays formatted, stores raw digits
            <div class="relative flex-1">
                <Input
                    class="pr-8 w-full rounded-l-none"
                    attr:r#type="tel"
                    attr:inputmode="numeric"
                    attr:placeholder=move || PhoneFormat::for_country(selected_country.get()).placeholder()
                    attr:disabled=disabled
                    attr:aria-label="Phone number"
                    attr:aria-invalid=move || invalid.get().unwrap_or(false)
                    prop:value=move || value.get().format(selected_country.get())
                    on:input=move |ev| {
                        let format = PhoneFormat::for_country(selected_country.get());
                        let phone = PhoneNumber::new(&event_target_value(&ev), format.max_digits);
                        value.set(phone);
                    }
                    on:blur=move |_| {
                        if let Some(cb) = on_blur {
                            cb.run(());
                        }
                    }
                />
                <Show when=move || !value.get().is_empty() && !disabled>
                    <button
                        type="button"
                        tabindex="-1"
                        class="absolute right-2 top-1/2 p-0.5 rounded-sm transition-colors -translate-y-1/2 text-muted-foreground hover:text-foreground hover:bg-muted"
                        aria-label="Clear phone number"
                        on:click=move |_| value.set(PhoneNumber::default())
                    >
                        <X class="size-4" />
                    </button>
                </Show>
            </div>
        </InputPhoneWrapper>
    }
}
```
