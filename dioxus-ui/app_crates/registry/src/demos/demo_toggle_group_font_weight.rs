use dioxus::prelude::*;

use crate::ui::toggle_group::{ToggleGroup, ToggleGroupItem};

const WEIGHTS: &[(&str, &str)] = &[
    ("100", "font-thin"),
    ("200", "font-extralight"),
    ("300", "font-light"),
    ("400", "font-normal"),
    ("500", "font-medium"),
    ("600", "font-semibold"),
    ("700", "font-bold"),
    ("800", "font-extrabold"),
    ("900", "font-black"),
];

#[component]
pub fn DemoToggleGroupFontWeight() -> Element {
    let mut selected = use_signal(|| "400");

    rsx! {
        div { class: "flex flex-col gap-4 items-center",
            ToggleGroup { spacing: 2,
                for (weight, _class) in WEIGHTS.iter() {
                    {
                        let w = *weight;
                        let font_style = format!("font-weight: {w}");
                        rsx! {
                            ToggleGroupItem {
                                title: format!("Weight {w}"),
                                pressed: selected() == w,
                                onclick: move |_| selected.set(w),
                                div { class: "flex flex-col gap-0.5 items-center py-1",
                                    span { class: "text-base leading-none", style: "{font_style}", "Aa" }
                                    span { class: "leading-none text-[10px] text-muted-foreground", "{w}" }
                                }
                            }
                        }
                    }
                }
            }
            p { class: "text-sm text-muted-foreground",
                "Selected: "
                code { class: "font-mono text-foreground",
                    {WEIGHTS.iter().find(|(w, _)| *w == selected()).map(|(_, c)| *c).unwrap_or("")}
                }
            }
        }
    }
}
