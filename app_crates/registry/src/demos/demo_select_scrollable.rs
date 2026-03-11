use leptos::prelude::*;

use crate::ui::select::{Select, SelectContent, SelectGroup, SelectLabel, SelectOption, SelectTrigger, SelectValue};

#[component]
pub fn DemoSelectScrollable() -> impl IntoView {
    view! {
        <Select>
            <SelectTrigger class="w-[280px]">
                <SelectValue placeholder="Select a timezone" />
            </SelectTrigger>

            <SelectContent class="w-[280px]">
                // North America
                <SelectGroup aria_label="North America">
                    <SelectLabel>"North America"</SelectLabel>
                    {[
                        ("est", "Eastern Standard Time (EST)"),
                        ("cst", "Central Standard Time (CST)"),
                        ("mst", "Mountain Standard Time (MST)"),
                        ("pst", "Pacific Standard Time (PST)"),
                        ("akst", "Alaska Standard Time (AKST)"),
                        ("hst", "Hawaii Standard Time (HST)"),
                    ]
                        .into_iter()
                        .map(|(value, label)| {
                            view! { <SelectOption value=value>{label}</SelectOption> }
                        })
                        .collect_view()}
                </SelectGroup>

                // Europe & Africa
                <SelectGroup aria_label="Europe & Africa">
                    <SelectLabel>"Europe & Africa"</SelectLabel>
                    {[
                        ("gmt", "Greenwich Mean Time (GMT)"),
                        ("cet", "Central European Time (CET)"),
                        ("eet", "Eastern European Time (EET)"),
                        ("west", "Western European Summer Time (WEST)"),
                        ("cat", "Central Africa Time (CAT)"),
                        ("eat", "East Africa Time (EAT)"),
                    ]
                        .into_iter()
                        .map(|(value, label)| {
                            view! { <SelectOption value=value>{label}</SelectOption> }
                        })
                        .collect_view()}
                </SelectGroup>

                // Asia
                <SelectGroup aria_label="Asia">
                    <SelectLabel>"Asia"</SelectLabel>
                    {[
                        ("msk", "Moscow Time (MSK)"),
                        ("ist", "India Standard Time (IST)"),
                        ("cst_china", "China Standard Time (CST)"),
                        ("jst", "Japan Standard Time (JST)"),
                        ("kst", "Korea Standard Time (KST)"),
                        ("wita", "Indonesia Central Standard Time (WITA)"),
                    ]
                        .into_iter()
                        .map(|(value, label)| {
                            view! { <SelectOption value=value>{label}</SelectOption> }
                        })
                        .collect_view()}
                </SelectGroup>

                // Australia & Pacific
                <SelectGroup aria_label="Australia & Pacific">
                    <SelectLabel>"Australia & Pacific"</SelectLabel>
                    {[
                        ("awst", "Australian Western Standard Time (AWST)"),
                        ("acst", "Australian Central Standard Time (ACST)"),
                        ("aest", "Australian Eastern Standard Time (AEST)"),
                        ("nzst", "New Zealand Standard Time (NZST)"),
                        ("fjt", "Fiji Time (FJT)"),
                    ]
                        .into_iter()
                        .map(|(value, label)| {
                            view! { <SelectOption value=value>{label}</SelectOption> }
                        })
                        .collect_view()}
                </SelectGroup>

                // South America
                <SelectGroup aria_label="South America">
                    <SelectLabel>"South America"</SelectLabel>
                    {[
                        ("art", "Argentina Time (ART)"),
                        ("bot", "Bolivia Time (BOT)"),
                        ("brt", "Brasilia Time (BRT)"),
                        ("clt", "Chile Standard Time (CLT)"),
                    ]
                        .into_iter()
                        .map(|(value, label)| {
                            view! { <SelectOption value=value>{label}</SelectOption> }
                        })
                        .collect_view()}
                </SelectGroup>
            </SelectContent>
        </Select>
    }
}
