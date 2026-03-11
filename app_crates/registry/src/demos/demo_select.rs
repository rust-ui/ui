use leptos::prelude::*;

use crate::ui::select::{Select, SelectContent, SelectGroup, SelectOption, SelectTrigger, SelectValue};

const OPTIONS: [&str; 3] = ["Components", "Extensions", "Icons"];

#[component]
pub fn DemoSelect() -> impl IntoView {
    view! {
        <Select>
            <SelectTrigger class="w-[150px]">
                <SelectValue placeholder="Please select" />
            </SelectTrigger>

            <SelectContent>
                <SelectGroup>
                    {OPTIONS
                        .into_iter()
                        .map(|option| {
                            view! { <SelectOption value=option>{option}</SelectOption> }
                        })
                        .collect_view()}
                </SelectGroup>
            </SelectContent>
        </Select>
    }
}
