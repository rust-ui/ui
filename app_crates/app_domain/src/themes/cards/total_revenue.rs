use dioxus::prelude::*;
use registry::ui::card::{Card, CardContent, CardHeader, CardTitle};

#[component]
pub fn CardTotalRevenue() -> Element {
    rsx! {
        Card {
            CardHeader { class: "flex-row justify-between items-center pb-2",
                CardTitle { class: "text-base font-normal", "Total Revenue" }
            }
            CardContent { class: "pt-0",
                div { class: "text-2xl font-bold", "$15,231.89" }
                p { class: "text-xs text-muted-foreground", "+20.1% from last month" }
                div { class: "h-[80px]",
                    ChartTotalRevenue {}
                }
            }
        }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn ChartTotalRevenue() -> Element {
    let values = r#"[186,305,237,273,209,237,264,486]"#;
    let labels = r#"["","","","","","","",""]"#;

    rsx! {
        div {
            class: "w-full h-full",
            "data-name": "LineChart",
            "data-chart-values": values,
            "data-chart-labels": labels,
            "data-chart-sparkline": "true",
            "data-chart-markers": "true",
        }
    }
}
