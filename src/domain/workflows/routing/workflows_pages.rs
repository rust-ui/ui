use app_config::SeoMeta;
use dioxus::prelude::*;

use crate::__registry__::all_workflows::ALL_WORKFLOWS;
use crate::domain::workflows::components::workflow_viewer::WorkflowViewer;

#[component]
pub fn WorkflowsPage() -> Element {
    rsx! {
        SeoMeta {
            title: "Workflows · Rust UI Component Library | Rust/UI".to_string(),
            description: "Ready-made workflow blocks built with Rust/UI components for Dioxus applications."
                .to_string(),
        }
        for workflow in ALL_WORKFLOWS {
            WorkflowViewer { workflow_entry: *workflow }
        }
    }
}
