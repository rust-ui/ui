use dioxus::prelude::*;
use registry::ui::card::{Card, CardContent, CardHeader, CardTitle};

#[component]
pub fn CardSubscriptions() -> Element {
    rsx! {
        Card {
            CardHeader { class: "flex-row justify-between items-center pb-2",
                CardTitle { class: "text-base font-normal", "Subscriptions" }
            }
            CardContent { class: "pt-0",
                div { class: "text-2xl font-bold", "+2350" }
                p { class: "text-xs text-muted-foreground", "+180.1% from last month" }
                div { class: "mt-4 h-[80px]",
                    ChartSubscriptions {}
                }
            }
        }
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn ChartSubscriptions() -> Element {
    let values = r#"[560,700,467,649,441,558,649,441]"#;
    let labels = r#"["","","","","","","",""]"#;

    rsx! {
        div {
            id: "chartSubscriptions",
            class: "w-full h-full",
            "data-name": "BarChart",
            "data-chart-values": values,
            "data-chart-labels": labels,
            "data-chart-show-xaxis": "false",
            "data-chart-show-yaxis": "false",
        }
    }
}
