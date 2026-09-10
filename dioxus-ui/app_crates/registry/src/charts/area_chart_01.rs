use dioxus::prelude::*;
use serde::Serialize;
use strum::{AsRefStr, Display, EnumIter, IntoEnumIterator};

use crate::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::ui::charts::AreaChart;

/*
 * title: Area Chart
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, AsRefStr, EnumIter, Default)]
enum TimeRange {
    #[default]
    #[strum(to_string = "Last 3 months")]
    Last3Months,
    #[strum(to_string = "Last 30 days")]
    Last30Days,
    #[strum(to_string = "Last 7 days")]
    Last7Days,
}

impl TimeRange {
    const fn to_days(self) -> usize {
        match self {
            Self::Last3Months => 90,
            Self::Last30Days => 30,
            Self::Last7Days => 7,
        }
    }
}

#[derive(Serialize)]
struct ChartDataPoint {
    date: &'static str,
    desktop: i32,
    mobile: i32,
}

const CHART_DATA: &[ChartDataPoint] = &[
    ChartDataPoint { date: "2024-04-01", desktop: 222, mobile: 150 },
    ChartDataPoint { date: "2024-04-02", desktop: 197, mobile: 97 },
    ChartDataPoint { date: "2024-04-03", desktop: 167, mobile: 120 },
    ChartDataPoint { date: "2024-04-04", desktop: 242, mobile: 180 },
    ChartDataPoint { date: "2024-04-05", desktop: 373, mobile: 290 },
    ChartDataPoint { date: "2024-04-06", desktop: 301, mobile: 240 },
    ChartDataPoint { date: "2024-04-07", desktop: 245, mobile: 180 },
    ChartDataPoint { date: "2024-04-08", desktop: 409, mobile: 320 },
    ChartDataPoint { date: "2024-04-09", desktop: 159, mobile: 59 },
    ChartDataPoint { date: "2024-04-10", desktop: 261, mobile: 190 },
    ChartDataPoint { date: "2024-04-11", desktop: 327, mobile: 250 },
    ChartDataPoint { date: "2024-04-12", desktop: 292, mobile: 210 },
    ChartDataPoint { date: "2024-04-13", desktop: 342, mobile: 280 },
    ChartDataPoint { date: "2024-04-14", desktop: 237, mobile: 137 },
    ChartDataPoint { date: "2024-04-15", desktop: 220, mobile: 120 },
    ChartDataPoint { date: "2024-04-16", desktop: 238, mobile: 138 },
    ChartDataPoint { date: "2024-04-17", desktop: 446, mobile: 360 },
    ChartDataPoint { date: "2024-04-18", desktop: 364, mobile: 310 },
    ChartDataPoint { date: "2024-04-19", desktop: 243, mobile: 180 },
    ChartDataPoint { date: "2024-04-20", desktop: 189, mobile: 89 },
    ChartDataPoint { date: "2024-04-21", desktop: 237, mobile: 137 },
    ChartDataPoint { date: "2024-04-22", desktop: 224, mobile: 170 },
    ChartDataPoint { date: "2024-04-23", desktop: 238, mobile: 138 },
    ChartDataPoint { date: "2024-04-24", desktop: 387, mobile: 290 },
    ChartDataPoint { date: "2024-04-25", desktop: 315, mobile: 215 },
    ChartDataPoint { date: "2024-04-26", desktop: 175, mobile: 75 },
    ChartDataPoint { date: "2024-04-27", desktop: 383, mobile: 320 },
    ChartDataPoint { date: "2024-04-28", desktop: 222, mobile: 122 },
    ChartDataPoint { date: "2024-04-29", desktop: 315, mobile: 240 },
    ChartDataPoint { date: "2024-04-30", desktop: 454, mobile: 380 },
    ChartDataPoint { date: "2024-05-01", desktop: 265, mobile: 165 },
    ChartDataPoint { date: "2024-05-02", desktop: 293, mobile: 230 },
    ChartDataPoint { date: "2024-05-03", desktop: 247, mobile: 190 },
    ChartDataPoint { date: "2024-05-04", desktop: 385, mobile: 320 },
    ChartDataPoint { date: "2024-05-05", desktop: 481, mobile: 390 },
    ChartDataPoint { date: "2024-05-06", desktop: 498, mobile: 420 },
    ChartDataPoint { date: "2024-05-07", desktop: 388, mobile: 300 },
    ChartDataPoint { date: "2024-05-08", desktop: 249, mobile: 149 },
    ChartDataPoint { date: "2024-05-09", desktop: 227, mobile: 180 },
    ChartDataPoint { date: "2024-05-10", desktop: 293, mobile: 230 },
    ChartDataPoint { date: "2024-05-11", desktop: 335, mobile: 270 },
    ChartDataPoint { date: "2024-05-12", desktop: 297, mobile: 197 },
    ChartDataPoint { date: "2024-05-13", desktop: 297, mobile: 197 },
    ChartDataPoint { date: "2024-05-14", desktop: 448, mobile: 390 },
    ChartDataPoint { date: "2024-05-15", desktop: 473, mobile: 380 },
    ChartDataPoint { date: "2024-05-16", desktop: 338, mobile: 280 },
    ChartDataPoint { date: "2024-05-17", desktop: 499, mobile: 420 },
    ChartDataPoint { date: "2024-05-18", desktop: 315, mobile: 255 },
    ChartDataPoint { date: "2024-05-19", desktop: 335, mobile: 235 },
    ChartDataPoint { date: "2024-05-20", desktop: 277, mobile: 177 },
    ChartDataPoint { date: "2024-05-21", desktop: 182, mobile: 82 },
    ChartDataPoint { date: "2024-05-22", desktop: 181, mobile: 81 },
    ChartDataPoint { date: "2024-05-23", desktop: 352, mobile: 252 },
    ChartDataPoint { date: "2024-05-24", desktop: 394, mobile: 294 },
    ChartDataPoint { date: "2024-05-25", desktop: 301, mobile: 201 },
    ChartDataPoint { date: "2024-05-26", desktop: 313, mobile: 213 },
    ChartDataPoint { date: "2024-05-27", desktop: 420, mobile: 360 },
    ChartDataPoint { date: "2024-05-28", desktop: 333, mobile: 233 },
    ChartDataPoint { date: "2024-05-29", desktop: 178, mobile: 78 },
    ChartDataPoint { date: "2024-05-30", desktop: 340, mobile: 280 },
    ChartDataPoint { date: "2024-05-31", desktop: 278, mobile: 178 },
    ChartDataPoint { date: "2024-06-01", desktop: 278, mobile: 178 },
    ChartDataPoint { date: "2024-06-02", desktop: 470, mobile: 410 },
    ChartDataPoint { date: "2024-06-03", desktop: 203, mobile: 103 },
    ChartDataPoint { date: "2024-06-04", desktop: 439, mobile: 380 },
    ChartDataPoint { date: "2024-06-05", desktop: 188, mobile: 88 },
    ChartDataPoint { date: "2024-06-06", desktop: 294, mobile: 250 },
    ChartDataPoint { date: "2024-06-07", desktop: 323, mobile: 270 },
    ChartDataPoint { date: "2024-06-08", desktop: 385, mobile: 320 },
    ChartDataPoint { date: "2024-06-09", desktop: 438, mobile: 380 },
    ChartDataPoint { date: "2024-06-10", desktop: 255, mobile: 155 },
    ChartDataPoint { date: "2024-06-11", desktop: 192, mobile: 92 },
    ChartDataPoint { date: "2024-06-12", desktop: 492, mobile: 420 },
    ChartDataPoint { date: "2024-06-13", desktop: 181, mobile: 81 },
    ChartDataPoint { date: "2024-06-14", desktop: 426, mobile: 380 },
    ChartDataPoint { date: "2024-06-15", desktop: 307, mobile: 250 },
    ChartDataPoint { date: "2024-06-16", desktop: 371, mobile: 310 },
    ChartDataPoint { date: "2024-06-17", desktop: 475, mobile: 420 },
    ChartDataPoint { date: "2024-06-18", desktop: 207, mobile: 107 },
    ChartDataPoint { date: "2024-06-19", desktop: 341, mobile: 290 },
    ChartDataPoint { date: "2024-06-20", desktop: 408, mobile: 350 },
    ChartDataPoint { date: "2024-06-21", desktop: 269, mobile: 169 },
    ChartDataPoint { date: "2024-06-22", desktop: 317, mobile: 270 },
    ChartDataPoint { date: "2024-06-23", desktop: 480, mobile: 430 },
    ChartDataPoint { date: "2024-06-24", desktop: 232, mobile: 132 },
    ChartDataPoint { date: "2024-06-25", desktop: 241, mobile: 141 },
    ChartDataPoint { date: "2024-06-26", desktop: 434, mobile: 380 },
    ChartDataPoint { date: "2024-06-27", desktop: 448, mobile: 390 },
    ChartDataPoint { date: "2024-06-28", desktop: 249, mobile: 149 },
    ChartDataPoint { date: "2024-06-29", desktop: 203, mobile: 103 },
    ChartDataPoint { date: "2024-06-30", desktop: 446, mobile: 400 },
];

#[component]
pub fn AreaChart01() -> Element {
    let mut time_range = use_signal(TimeRange::default);

    let json_values = use_memo(move || {
        let range = time_range();
        let days = range.to_days();
        let start = CHART_DATA.len().saturating_sub(days);
        let data = CHART_DATA.get(start..).unwrap_or(CHART_DATA);
        let desktop: Vec<i32> = data.iter().map(|d| d.desktop).collect();
        let mobile: Vec<i32> = data.iter().map(|d| d.mobile).collect();
        serde_json::to_string(&[desktop, mobile]).unwrap_or_default()
    });

    let json_labels = use_memo(move || {
        let range = time_range();
        let days = range.to_days();
        let start = CHART_DATA.len().saturating_sub(days);
        let data = CHART_DATA.get(start..).unwrap_or(CHART_DATA);
        let labels: Vec<&str> = data.iter().map(|d| d.date).collect();
        serde_json::to_string(&labels).unwrap_or_default()
    });

    let series_names_json = serde_json::to_string(&["Desktop", "Mobile"]).unwrap_or_default();

    rsx! {
        div { class: "flex overflow-hidden relative flex-col rounded-xl border transition-all duration-200 ease-in-out hover:z-30 group",
            div { class: "flex gap-2 justify-between items-center py-2.5 px-3 border-b bg-card text-card-foreground",
                div { class: "flex gap-1.5 items-center pl-1 text-muted-foreground text-[13px]",
                    span { "Area Chart" }
                }
                div { class: "flex gap-1",
                    for range in TimeRange::iter() {
                        button {
                            class: {
                                let is_active = time_range() == range;
                                if is_active {
                                    "px-2 py-1 text-xs rounded-md bg-primary text-primary-foreground"
                                } else {
                                    "px-2 py-1 text-xs rounded-md text-muted-foreground hover:text-foreground"
                                }
                            },
                            onclick: move |_| time_range.set(range),
                            "{range}"
                        }
                    }
                }
            }
            Card { class: "gap-0 py-0 rounded-none border-0 shadow-none",
                CardHeader { class: "py-6 border-b",
                    CardTitle { "Area Chart - Interactive" }
                    CardDescription { "Showing total visitors for the last 3 months" }
                }
                CardContent { class: "px-2 pt-4 sm:px-6 sm:pt-6",
                    AreaChart {
                        class: "h-[250px]",
                        json_values: json_values(),
                        json_labels: json_labels(),
                        series_names: series_names_json,
                    }
                }
            }
        }
    }
}
