use std::collections::HashSet;

use dioxus::prelude::*;

use crate::ui::multi_select::{
    MultiSelect, MultiSelectContent, MultiSelectGroup, MultiSelectItem, MultiSelectOption, MultiSelectTrigger,
    MultiSelectValue,
};

const FRUITS: &[&str] = &["Apple", "Banana", "Orange", "Strawberry", "Mango"];

#[component]
pub fn DemoMultiSelect() -> Element {
    let fruits_signal = use_signal(HashSet::<String>::new);

    rsx! {
        div { class: "flex flex-col gap-4",
            span { class: "text-sm",
                {
                    let values = fruits_signal();
                    if values.is_empty() {
                        "No fruits selected".to_string()
                    } else {
                        format!("Selected: {}", values.iter().cloned().collect::<Vec<_>>().join(", "))
                    }
                }
            }

            div { class: "mx-auto",
                MultiSelect { values: fruits_signal,
                    MultiSelectTrigger { class: "w-[150px]",
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
