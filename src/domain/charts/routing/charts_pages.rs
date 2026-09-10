use dioxus::prelude::*;
use registry::charts::area_chart_01::AreaChart01;
use registry::charts::area_chart_02::AreaChart02;
use registry::charts::area_chart_03::AreaChart03;
use registry::charts::area_chart_04::AreaChart04;
use registry::charts::area_chart_05::AreaChart05;
use registry::charts::area_chart_06::AreaChart06;
use registry::charts::area_chart_07::AreaChart07;
use registry::charts::area_chart_08::AreaChart08;
use registry::charts::area_chart_09::AreaChart09;
use registry::charts::area_chart_10::AreaChart10;
use registry::charts::area_chart_11::AreaChart11;
use registry::charts::area_chart_placeholder::AreaChartPlaceholder;
use registry::charts::bar_chart_01::BarChart01;
use registry::charts::line_chart_01::LineChart01;
use registry::charts::pie_chart_01::PieChart01;
use registry::charts::radar_chart_01::RadarChart01;
use registry::charts::radial_chart_01::RadialChart01;

#[component]
pub fn AreaChartPage() -> Element {
    rsx! {
        div { class: "flex flex-col gap-8",
            AreaChart01 {}

            div { class: "grid grid-cols-1 gap-6 sm:grid-cols-2 md:grid-cols-3",
                div { class: "[content-visibility:auto] [contain-intrinsic-size:400px]", AreaChart02 {} }
                div { class: "[content-visibility:auto] [contain-intrinsic-size:400px]", AreaChart03 {} }
                div { class: "[content-visibility:auto] [contain-intrinsic-size:400px]", AreaChart04 {} }
                div { class: "[content-visibility:auto] [contain-intrinsic-size:400px]", AreaChart05 {} }
                div { class: "[content-visibility:auto] [contain-intrinsic-size:400px]", AreaChart06 {} }
                div { class: "[content-visibility:auto] [contain-intrinsic-size:400px]", AreaChart07 {} }
                div { class: "[content-visibility:auto] [contain-intrinsic-size:400px]", AreaChart08 {} }
                div { class: "[content-visibility:auto] [contain-intrinsic-size:400px]", AreaChart09 {} }
                div { class: "[content-visibility:auto] [contain-intrinsic-size:400px]", AreaChart10 {} }
                div { class: "[content-visibility:auto] [contain-intrinsic-size:400px]", AreaChart11 {} }
                div { class: "[content-visibility:auto] [contain-intrinsic-size:400px]", AreaChartPlaceholder {} }
                div { class: "[content-visibility:auto] [contain-intrinsic-size:400px]", AreaChartPlaceholder {} }
            }
        }
    }
}

#[component]
pub fn BarChartPage() -> Element {
    rsx! { BarChart01 {} }
}

#[component]
pub fn LineChartPage() -> Element {
    rsx! { LineChart01 {} }
}

#[component]
pub fn PieChartPage() -> Element {
    rsx! { PieChart01 {} }
}

#[component]
pub fn RadarChartPage() -> Element {
    rsx! { RadarChart01 {} }
}

#[component]
pub fn RadialChartPage() -> Element {
    rsx! { RadialChart01 {} }
}
