use dioxus::prelude::*;
use icons::{ChartArea, TrendingUp};
use serde::Serialize;

use crate::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
use crate::ui::charts::AreaChart;

/*
 * title: Area Chart - Stacked Expanded
*/

#[derive(Serialize)]
struct ChartDataPoint {
    month: &'static str,
    series1: i32,
    series2: i32,
    series3: i32,
}

const CHART_DATA: &[ChartDataPoint] = &[
    ChartDataPoint { month: "Jan", series1: 30, series2: 40, series3: 30 },
    ChartDataPoint { month: "Feb", series1: 35, series2: 50, series3: 25 },
    ChartDataPoint { month: "Mar", series1: 25, series2: 45, series3: 35 },
    ChartDataPoint { month: "Apr", series1: 40, series2: 55, series3: 20 },
    ChartDataPoint { month: "May", series1: 35, series2: 40, series3: 30 },
    ChartDataPoint { month: "Jun", series1: 30, series2: 45, series3: 35 },
];

#[component]
pub fn AreaChart07() -> Element {
    let s1: Vec<i32> = CHART_DATA.iter().map(|d| d.series1).collect();
    let s2: Vec<i32> = CHART_DATA.iter().map(|d| d.series2).collect();
    let s3: Vec<i32> = CHART_DATA.iter().map(|d| d.series3).collect();
    let labels: Vec<&str> = CHART_DATA.iter().map(|d| d.month).collect();

    let json_values = serde_json::to_string(&[s1, s2, s3]).unwrap_or_default();
    let json_labels = serde_json::to_string(&labels).unwrap_or_default();

    rsx! {
        div { class: "flex relative flex-col rounded-xl border transition-all duration-200 ease-in-out hover:z-30 group",
            div { class: "flex gap-2 justify-start items-center py-2.5 px-3 border-b bg-card text-card-foreground",
                div { class: "flex gap-1.5 items-center pl-1 text-muted-foreground text-[13px] [&>svg]:h-[0.9rem] [&>svg]:w-[0.9rem]",
                    ChartArea {}
                    span { "Area Chart" }
                }
            }
            Card { class: "gap-0 py-0 rounded-none border-0 shadow-none",
                CardHeader { class: "py-6",
                    CardTitle { "Area Chart - Stacked Expanded" }
                    CardDescription { "Showing total visitors for the last 6 months" }
                }
                CardContent { class: "px-2",
                    AreaChart { class: "h-[250px]", json_values, json_labels, stack_type: "100%" }
                }
                CardFooter { class: "flex-col gap-2 items-start py-6 text-sm",
                    div { class: "flex gap-2 items-center font-medium leading-none",
                        span { "Trending up by 5.2% this month" }
                        TrendingUp { class: "size-4" }
                    }
                    div { class: "leading-none text-muted-foreground", "January - June 2024" }
                }
            }
        }
    }
}
