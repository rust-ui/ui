---
title: "Demo Multi Select Scrollable"
name: "demo_multi_select_scrollable"
cargo_dependencies: []
registry_dependencies: ["multi_select"]
type: "components:demos"
path: "demos/demo_multi_select_scrollable.rs"
---

# Demo Multi Select Scrollable

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_multi_select_scrollable
```

## Component Code

```rust
use std::collections::HashSet;

use leptos::prelude::*;

use crate::components::ui::multi_select::{
    MultiSelect, MultiSelectContent, MultiSelectGroup, MultiSelectItem, MultiSelectLabel, MultiSelectOption,
    MultiSelectTrigger, MultiSelectValue,
};

#[component]
pub fn DemoMultiSelectScrollable() -> impl IntoView {
    let selected_signal = RwSignal::new(HashSet::<String>::new());

    view! {
        <MultiSelect values=selected_signal>
            <MultiSelectTrigger class="w-[280px]">
                <MultiSelectValue placeholder="Select timezones" />
            </MultiSelectTrigger>

            <MultiSelectContent class="w-[280px]">
                // North America
                <MultiSelectGroup>
                    <MultiSelectLabel>"North America"</MultiSelectLabel>
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
                            view! {
                                <MultiSelectItem>
                                    <MultiSelectOption value=value>{label}</MultiSelectOption>
                                </MultiSelectItem>
                            }
                        })
                        .collect_view()}
                </MultiSelectGroup>

                // Europe & Africa
                <MultiSelectGroup>
                    <MultiSelectLabel>"Europe & Africa"</MultiSelectLabel>
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
                            view! {
                                <MultiSelectItem>
                                    <MultiSelectOption value=value>{label}</MultiSelectOption>
                                </MultiSelectItem>
                            }
                        })
                        .collect_view()}
                </MultiSelectGroup>

                // Asia
                <MultiSelectGroup>
                    <MultiSelectLabel>"Asia"</MultiSelectLabel>
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
                            view! {
                                <MultiSelectItem>
                                    <MultiSelectOption value=value>{label}</MultiSelectOption>
                                </MultiSelectItem>
                            }
                        })
                        .collect_view()}
                </MultiSelectGroup>

                // Australia & Pacific
                <MultiSelectGroup>
                    <MultiSelectLabel>"Australia & Pacific"</MultiSelectLabel>
                    {[
                        ("awst", "Australian Western Standard Time (AWST)"),
                        ("acst", "Australian Central Standard Time (ACST)"),
                        ("aest", "Australian Eastern Standard Time (AEST)"),
                        ("nzst", "New Zealand Standard Time (NZST)"),
                        ("fjt", "Fiji Time (FJT)"),
                    ]
                        .into_iter()
                        .map(|(value, label)| {
                            view! {
                                <MultiSelectItem>
                                    <MultiSelectOption value=value>{label}</MultiSelectOption>
                                </MultiSelectItem>
                            }
                        })
                        .collect_view()}
                </MultiSelectGroup>

                // South America
                <MultiSelectGroup>
                    <MultiSelectLabel>"South America"</MultiSelectLabel>
                    {[
                        ("art", "Argentina Time (ART)"),
                        ("bot", "Bolivia Time (BOT)"),
                        ("brt", "Brasilia Time (BRT)"),
                        ("clt", "Chile Standard Time (CLT)"),
                    ]
                        .into_iter()
                        .map(|(value, label)| {
                            view! {
                                <MultiSelectItem>
                                    <MultiSelectOption value=value>{label}</MultiSelectOption>
                                </MultiSelectItem>
                            }
                        })
                        .collect_view()}
                </MultiSelectGroup>
            </MultiSelectContent>
        </MultiSelect>
    }
}
```
