use icons::{ChartArea, TrendingUp};
use leptos::prelude::*;
use serde::Serialize;

use crate::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
use crate::ui::charts::AreaChart;

/*
 * title: Area Chart - No Y-axis
*/

#[derive(Serialize)]
struct ChartDataPoint {
    month: &'static str,
    desktop: i32,
    mobile: i32,
}

const CHART_DATA: &[ChartDataPoint] = &[
    ChartDataPoint { month: "Jan", desktop: 186, mobile: 80 },
    ChartDataPoint { month: "Feb", desktop: 305, mobile: 200 },
    ChartDataPoint { month: "Mar", desktop: 237, mobile: 120 },
    ChartDataPoint { month: "Apr", desktop: 73, mobile: 190 },
    ChartDataPoint { month: "May", desktop: 209, mobile: 130 },
    ChartDataPoint { month: "Jun", desktop: 214, mobile: 140 },
];

#[component]
pub fn AreaChart10() -> impl IntoView {
    let desktop_data: Vec<i32> = CHART_DATA.iter().map(|d| d.desktop).collect();
    let mobile_data: Vec<i32> = CHART_DATA.iter().map(|d| d.mobile).collect();
    let labels: Vec<&str> = CHART_DATA.iter().map(|d| d.month).collect();

    let chart_data = vec![desktop_data, mobile_data];
    let json_values = serde_json::to_string(&chart_data).unwrap_or_default();
    let json_labels = serde_json::to_string(&labels).unwrap_or_default();

    view! {
        <div class="flex relative flex-col rounded-xl border transition-all duration-200 ease-in-out hover:z-30 group">
            // Top header bar with icon and label
            <div class="flex gap-2 justify-start items-center py-2.5 px-3 border-b bg-card text-card-foreground">
                <div class="flex gap-1.5 items-center pl-1 text-muted-foreground text-[13px] [&>svg]:h-[0.9rem] [&>svg]:w-[0.9rem]">
                    <ChartArea />
                    <span>Area Chart</span>
                </div>
            </div>

            // Main Card
            <Card class="gap-0 py-0 rounded-none border-0 shadow-none">
                <CardHeader class="py-6">
                    <CardTitle>Area Chart - No Y-axis</CardTitle>
                    <CardDescription>Showing total visitors for the last 6 months</CardDescription>
                </CardHeader>

                <CardContent class="px-2">
                    // * AreaChart demo without Y-axis
                    <AreaChart class="h-[250px]" json_values json_labels show_yaxis=false />
                </CardContent>

                <CardFooter class="flex-col gap-2 items-start py-6 text-sm">
                    <div class="flex gap-2 items-center font-medium leading-none">
                        <span>Trending up by 5.2% this month</span>
                        <TrendingUp class="size-4" />
                    </div>
                    <div class="leading-none text-muted-foreground">January - June 2024</div>
                </CardFooter>
            </Card>
        </div>
    }
}
