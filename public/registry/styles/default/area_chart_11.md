---
title: "Area Chart 11"
name: "area_chart_11"
cargo_dependencies: []
registry_dependencies: ["card", "charts"]
type: "components:charts"
path: "charts/area_chart_11.rs"
---

# Area Chart 11

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add area_chart_11
```

## Component Code

```rust
use icons::ChartArea;
use leptos::prelude::*;
use serde::Serialize;

use crate::components::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
use crate::components::ui::charts::AreaChart;

#[derive(Serialize)]
struct ChartDataPoint {
    date: &'static str,
    visitors: i32,
}

#[derive(Serialize)]
struct ArticleAnnotation {
    x: &'static str,
    title: &'static str,
}

const CHART_DATA: &[ChartDataPoint] = &[
    ChartDataPoint { date: "2024-01-01", visitors: 142 },
    ChartDataPoint { date: "2024-01-03", visitors: 189 },
    ChartDataPoint { date: "2024-01-05", visitors: 156 },
    ChartDataPoint { date: "2024-01-07", visitors: 210 },
    ChartDataPoint { date: "2024-01-09", visitors: 287 },
    ChartDataPoint { date: "2024-01-11", visitors: 324 },
    ChartDataPoint { date: "2024-01-13", visitors: 298 },
    ChartDataPoint { date: "2024-01-15", visitors: 412 },
    ChartDataPoint { date: "2024-01-17", visitors: 376 },
    ChartDataPoint { date: "2024-01-19", visitors: 340 },
    ChartDataPoint { date: "2024-01-21", visitors: 298 },
    ChartDataPoint { date: "2024-01-23", visitors: 265 },
    ChartDataPoint { date: "2024-01-25", visitors: 389 },
    ChartDataPoint { date: "2024-01-27", visitors: 445 },
    ChartDataPoint { date: "2024-01-29", visitors: 398 },
];

const ARTICLES: &[ArticleAnnotation] = &[
    ArticleAnnotation { x: "2024-01-09", title: "Getting Started with Rust" },
    ArticleAnnotation { x: "2024-01-15", title: "Rust Async Programming with Tokio" },
    ArticleAnnotation { x: "2024-01-27", title: "Building REST APIs with Axum" },
];

#[component]
pub fn AreaChart11() -> impl IntoView {
    let visitors: Vec<i32> = CHART_DATA.iter().map(|d| d.visitors).collect();
    let labels: Vec<&str> = CHART_DATA.iter().map(|d| d.date).collect();
    let json_values = serde_json::to_string(&[visitors]).unwrap_or_default();
    let json_labels = serde_json::to_string(&labels).unwrap_or_default();
    let json_annotations = serde_json::to_string(ARTICLES).unwrap_or_default();

    view! {
        <div class="flex overflow-hidden relative flex-col rounded-xl border transition-all duration-200 ease-in-out hover:z-30 group">
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
                    <CardTitle>Area Chart - Article Markers</CardTitle>
                    <CardDescription>
                        "Hover the "
                        <span class="text-blue-500 font-medium">"◆"</span>
                        " markers to see which article was published"
                    </CardDescription>
                </CardHeader>

                <CardContent class="px-2 pt-4 sm:px-6 sm:pt-6">
                    <AreaChart
                        class="h-[250px]"
                        json_values
                        json_labels
                        series_names="[\"Visitors\"]"
                        gradient=true
                        show_grid=true
                        show_yaxis=true
                        json_annotations=json_annotations
                    />
                </CardContent>

                <CardFooter class="flex-col gap-2 items-start py-6 text-sm">
                    <div class="leading-none text-muted-foreground">
                        "◆ markers indicate article publish dates — hover to see the title"
                    </div>
                </CardFooter>
            </Card>
        </div>
    }
}
```
