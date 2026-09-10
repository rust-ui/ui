use dioxus::prelude::*;

use crate::ui::select::{Select, SelectContent, SelectGroup, SelectLabel, SelectOption, SelectTrigger, SelectValue};

#[component]
pub fn DemoSelectScrollable() -> Element {
    rsx! {
        Select {
            SelectTrigger { class: "w-[280px]",
                SelectValue { placeholder: "Select a timezone" }
            }

            SelectContent { class: "w-[280px]",
                // North America
                SelectGroup { aria_label: "North America",
                    SelectLabel { "North America" }
                    SelectOption { value: "est", "Eastern Standard Time (EST)" }
                    SelectOption { value: "cst", "Central Standard Time (CST)" }
                    SelectOption { value: "mst", "Mountain Standard Time (MST)" }
                    SelectOption { value: "pst", "Pacific Standard Time (PST)" }
                    SelectOption { value: "akst", "Alaska Standard Time (AKST)" }
                    SelectOption { value: "hst", "Hawaii Standard Time (HST)" }
                }

                // Europe & Africa
                SelectGroup { aria_label: "Europe & Africa",
                    SelectLabel { "Europe & Africa" }
                    SelectOption { value: "gmt", "Greenwich Mean Time (GMT)" }
                    SelectOption { value: "cet", "Central European Time (CET)" }
                    SelectOption { value: "eet", "Eastern European Time (EET)" }
                    SelectOption { value: "west", "Western European Summer Time (WEST)" }
                    SelectOption { value: "cat", "Central Africa Time (CAT)" }
                    SelectOption { value: "eat", "East Africa Time (EAT)" }
                }

                // Asia
                SelectGroup { aria_label: "Asia",
                    SelectLabel { "Asia" }
                    SelectOption { value: "msk", "Moscow Time (MSK)" }
                    SelectOption { value: "ist", "India Standard Time (IST)" }
                    SelectOption { value: "cst_china", "China Standard Time (CST)" }
                    SelectOption { value: "jst", "Japan Standard Time (JST)" }
                    SelectOption { value: "kst", "Korea Standard Time (KST)" }
                    SelectOption { value: "ist_indo", "Indonesia Central Standard Time (WITA)" }
                }

                // Australia & Pacific
                SelectGroup { aria_label: "Australia & Pacific",
                    SelectLabel { "Australia & Pacific" }
                    SelectOption { value: "awst", "Australian Western Standard Time (AWST)" }
                    SelectOption { value: "acst", "Australian Central Standard Time (ACST)" }
                    SelectOption { value: "aest", "Australian Eastern Standard Time (AEST)" }
                    SelectOption { value: "nzst", "New Zealand Standard Time (NZST)" }
                    SelectOption { value: "fjt", "Fiji Time (FJT)" }
                }

                // South America
                SelectGroup { aria_label: "South America",
                    SelectLabel { "South America" }
                    SelectOption { value: "art", "Argentina Time (ART)" }
                    SelectOption { value: "bot", "Bolivia Time (BOT)" }
                    SelectOption { value: "brt", "Brasilia Time (BRT)" }
                    SelectOption { value: "clt", "Chile Standard Time (CLT)" }
                }
            }
        }
    }
}
