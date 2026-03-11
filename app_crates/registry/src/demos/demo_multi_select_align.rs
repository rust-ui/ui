use std::collections::HashSet;

use leptos::prelude::*;

use crate::ui::multi_select::{
    MultiSelect, MultiSelectAlign, MultiSelectContent, MultiSelectGroup, MultiSelectItem, MultiSelectOption,
    MultiSelectTrigger, MultiSelectValue,
};

const FRUITS: [&str; 5] = ["Apple", "Banana", "Orange", "Strawberry", "Mango"];

#[component]
pub fn DemoMultiSelectAlign() -> impl IntoView {
    let start_signal = RwSignal::new(HashSet::<String>::new());
    let end_signal = RwSignal::new(HashSet::<String>::new());

    view! {
        <div class="flex justify-between mx-auto w-full max-w-[400px]">
            // Start alignment
            <div class="flex flex-col gap-2">
                <span class="text-sm text-muted-foreground">"Start"</span>
                <MultiSelect values=start_signal align=MultiSelectAlign::Start>
                    <MultiSelectTrigger class="w-fit">
                        <MultiSelectValue placeholder="Select fruits" />
                    </MultiSelectTrigger>

                    <MultiSelectContent>
                        <MultiSelectGroup>
                            {FRUITS
                                .into_iter()
                                .map(|fruit| {
                                    view! {
                                        <MultiSelectItem>
                                            <MultiSelectOption value=fruit>{fruit}</MultiSelectOption>
                                        </MultiSelectItem>
                                    }
                                })
                                .collect_view()}
                        </MultiSelectGroup>
                    </MultiSelectContent>
                </MultiSelect>
            </div>

            // End alignment
            <div class="flex flex-col gap-2 items-end">
                <span class="text-sm text-muted-foreground">"End"</span>
                <MultiSelect values=end_signal align=MultiSelectAlign::End>
                    <MultiSelectTrigger class="w-fit">
                        <MultiSelectValue placeholder="Select fruits" />
                    </MultiSelectTrigger>

                    <MultiSelectContent>
                        <MultiSelectGroup>
                            {FRUITS
                                .into_iter()
                                .map(|fruit| {
                                    view! {
                                        <MultiSelectItem>
                                            <MultiSelectOption value=fruit>{fruit}</MultiSelectOption>
                                        </MultiSelectItem>
                                    }
                                })
                                .collect_view()}
                        </MultiSelectGroup>
                    </MultiSelectContent>
                </MultiSelect>
            </div>
        </div>
    }
}
