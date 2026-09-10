use std::collections::HashSet;

use dioxus::prelude::*;

use crate::ui::multi_select::{
    MultiSelect, MultiSelectAlign, MultiSelectContent, MultiSelectGroup, MultiSelectItem, MultiSelectOption,
    MultiSelectTrigger, MultiSelectValue,
};

const FRUITS: &[&str] = &["Apple", "Banana", "Orange", "Strawberry", "Mango"];

#[component]
pub fn DemoMultiSelectAlign() -> Element {
    let start_signal = use_signal(HashSet::<String>::new);
    let end_signal = use_signal(HashSet::<String>::new);

    rsx! {
        div { class: "flex justify-between mx-auto w-full max-w-[400px]",
            // Start alignment
            div { class: "flex flex-col gap-2",
                span { class: "text-sm text-muted-foreground", "Start" }
                MultiSelect { values: start_signal, align: MultiSelectAlign::Start,
                    MultiSelectTrigger { class: "w-fit",
                        MultiSelectValue { placeholder: "Select fruits" }
                    }

                    MultiSelectContent {
                        MultiSelectGroup {
                            for fruit in FRUITS.iter() {
                                MultiSelectItem {
                                    MultiSelectOption { value: *fruit,
                                        {*fruit}
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // End alignment
            div { class: "flex flex-col gap-2 items-end",
                span { class: "text-sm text-muted-foreground", "End" }
                MultiSelect { values: end_signal, align: MultiSelectAlign::End,
                    MultiSelectTrigger { class: "w-fit",
                        MultiSelectValue { placeholder: "Select fruits" }
                    }

                    MultiSelectContent {
                        MultiSelectGroup {
                            for fruit in FRUITS.iter() {
                                MultiSelectItem {
                                    MultiSelectOption { value: *fruit,
                                        {*fruit}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
