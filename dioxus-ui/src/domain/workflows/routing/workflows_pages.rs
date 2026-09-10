use dioxus::prelude::*;

use crate::__registry__::all_workflows::ALL_WORKFLOWS;
use crate::domain::workflows::components::workflow_viewer::WorkflowViewer;

#[component]
pub fn WorkflowsPage() -> Element {
    rsx! {
        for workflow in ALL_WORKFLOWS {
            WorkflowViewer { workflow_entry: *workflow }
        }
    }
}
