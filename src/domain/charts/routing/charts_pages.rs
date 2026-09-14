use app_config::{BreadcrumbItem, JsonLdBreadcrumb, SeoMeta, SiteConfig};
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
        SeoMeta {
            title: "Area Chart · Rust UI Component Library | Rust/UI".to_string(),
            description: "Area chart components built with Rust/UI for Dioxus applications. Copy-paste examples ready to use."
                .to_string(),
            canonical_url: format!("{}/charts/area-chart", SiteConfig::BASE_URL),
        }
        JsonLdBreadcrumb {
            breadcrumbs: vec![
                BreadcrumbItem { name: "Home".to_string(), url: Some(SiteConfig::BASE_URL.to_string()) },
                BreadcrumbItem { name: "Area Chart".to_string(), url: None },
            ],
        }
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
    rsx! {
        SeoMeta {
            title: "Bar Chart · Rust UI Component Library | Rust/UI".to_string(),
            description: "Bar chart components built with Rust/UI for Dioxus applications. Copy-paste examples ready to use."
                .to_string(),
            canonical_url: format!("{}/charts/bar-chart", SiteConfig::BASE_URL),
        }
        JsonLdBreadcrumb {
            breadcrumbs: vec![
                BreadcrumbItem { name: "Home".to_string(), url: Some(SiteConfig::BASE_URL.to_string()) },
                BreadcrumbItem { name: "Bar Chart".to_string(), url: None },
            ],
        }
        BarChart01 {}
    }
}

#[component]
pub fn LineChartPage() -> Element {
    rsx! {
        SeoMeta {
            title: "Line Chart · Rust UI Component Library | Rust/UI".to_string(),
            description: "Line chart components built with Rust/UI for Dioxus applications. Copy-paste examples ready to use."
                .to_string(),
            canonical_url: format!("{}/charts/line-chart", SiteConfig::BASE_URL),
        }
        JsonLdBreadcrumb {
            breadcrumbs: vec![
                BreadcrumbItem { name: "Home".to_string(), url: Some(SiteConfig::BASE_URL.to_string()) },
                BreadcrumbItem { name: "Line Chart".to_string(), url: None },
            ],
        }
        LineChart01 {}
    }
}

#[component]
pub fn PieChartPage() -> Element {
    rsx! {
        SeoMeta {
            title: "Pie Chart · Rust UI Component Library | Rust/UI".to_string(),
            description: "Pie chart components built with Rust/UI for Dioxus applications. Copy-paste examples ready to use."
                .to_string(),
            canonical_url: format!("{}/charts/pie-chart", SiteConfig::BASE_URL),
        }
        JsonLdBreadcrumb {
            breadcrumbs: vec![
                BreadcrumbItem { name: "Home".to_string(), url: Some(SiteConfig::BASE_URL.to_string()) },
                BreadcrumbItem { name: "Pie Chart".to_string(), url: None },
            ],
        }
        PieChart01 {}
    }
}

#[component]
pub fn RadarChartPage() -> Element {
    rsx! {
        SeoMeta {
            title: "Radar Chart · Rust UI Component Library | Rust/UI".to_string(),
            description: "Radar chart components built with Rust/UI for Dioxus applications. Copy-paste examples ready to use."
                .to_string(),
            canonical_url: format!("{}/charts/radar-chart", SiteConfig::BASE_URL),
        }
        JsonLdBreadcrumb {
            breadcrumbs: vec![
                BreadcrumbItem { name: "Home".to_string(), url: Some(SiteConfig::BASE_URL.to_string()) },
                BreadcrumbItem { name: "Radar Chart".to_string(), url: None },
            ],
        }
        RadarChart01 {}
    }
}

#[component]
pub fn RadialChartPage() -> Element {
    rsx! {
        SeoMeta {
            title: "Radial Chart · Rust UI Component Library | Rust/UI".to_string(),
            description: "Radial chart components built with Rust/UI for Dioxus applications. Copy-paste examples ready to use."
                .to_string(),
            canonical_url: format!("{}/charts/radial-chart", SiteConfig::BASE_URL),
        }
        JsonLdBreadcrumb {
            breadcrumbs: vec![
                BreadcrumbItem { name: "Home".to_string(), url: Some(SiteConfig::BASE_URL.to_string()) },
                BreadcrumbItem { name: "Radial Chart".to_string(), url: None },
            ],
        }
        RadialChart01 {}
    }
}
