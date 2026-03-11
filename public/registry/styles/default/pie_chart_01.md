---
title: "Pie Chart 01"
name: "pie_chart_01"
cargo_dependencies: []
registry_dependencies: []
type: "components:charts"
path: "charts/pie_chart_01.rs"
---

# Pie Chart 01

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add pie_chart_01
```

## Component Code

```rust
use leptos::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
struct ChartDataPoint {
    label: &'static str,
    value: i32,
}

const CHART_DATA: &[ChartDataPoint] = &[
    ChartDataPoint { label: "Chrome", value: 44 },
    ChartDataPoint { label: "Safari", value: 25 },
    ChartDataPoint { label: "Firefox", value: 18 },
    ChartDataPoint { label: "Edge", value: 8 },
    ChartDataPoint { label: "Other", value: 5 },
];

#[component]
pub fn PieChart01() -> impl IntoView {
    let values: Vec<i32> = CHART_DATA.iter().map(|d| d.value).collect();
    let labels: Vec<&str> = CHART_DATA.iter().map(|d| d.label).collect();

    let data_json = serde_json::to_string(&values).unwrap_or_default();
    let labels_json = serde_json::to_string(&labels).unwrap_or_default();

    view! {
        <section class="flex flex-col gap-9 px-6 mx-auto w-full max-w-4xl rounded-lg border border-border bg-card">
            <p class="mt-4 text-xs font-bold text-card-foreground">Overview</p>

            <div
                id="pieChart01"
                class="w-full h-[400px]"
                data-name="PieChart"
                data-chart-values=data_json
                data-chart-labels=labels_json
            ></div>
        </section>
    }
}
```
