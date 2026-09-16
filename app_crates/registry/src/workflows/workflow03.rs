use dioxus::prelude::*;

use crate::hooks::use_workflow::{WorkflowEdge, WorkflowNode, WorkflowNodeKind, use_workflow};
use crate::ui::workflow::{WorkflowCanvas, WorkflowControls, WorkflowDefaultNode, WorkflowNodeWrapper};

fn initial_nodes() -> Vec<WorkflowNode> {
    vec![
        WorkflowNode {
            id: "a".to_string(),
            initial_x: 40.0,
            initial_y: 120.0,
            width: 192.0,
            has_target: false,
            has_source: true,
            label: "Step A".to_string(),
            description: "Select me, use ↑↓←→".to_string(),
            kind: WorkflowNodeKind::Trigger,
        },
        WorkflowNode {
            id: "b".to_string(),
            initial_x: 300.0,
            initial_y: 120.0,
            width: 192.0,
            has_target: true,
            has_source: true,
            label: "Step B".to_string(),
            description: "Arrow keys nudge 20px".to_string(),
            kind: WorkflowNodeKind::Agent,
        },
        WorkflowNode {
            id: "c".to_string(),
            initial_x: 560.0,
            initial_y: 120.0,
            width: 192.0,
            has_target: true,
            has_source: false,
            label: "Step C".to_string(),
            description: "Shift+click multi-select".to_string(),
            kind: WorkflowNodeKind::Output,
        },
    ]
}

fn initial_edges() -> Vec<WorkflowEdge> {
    vec![
        WorkflowEdge { from: "a".to_string(), to: "b".to_string(), ..Default::default() },
        WorkflowEdge { from: "b".to_string(), to: "c".to_string(), ..Default::default() },
    ]
}

#[component]
pub fn Workflow03() -> Element {
    let state = use_workflow(initial_nodes(), initial_edges());

    rsx! {
        WorkflowCanvas {
            state,
            overlay: rsx! { WorkflowControls { state } },
            for (i, node) in state.nodes.read().iter().cloned().enumerate() {
                WorkflowNodeWrapper { key: "{node.id}", state, idx: i,
                    WorkflowDefaultNode { node }
                }
            }
        }
    }
}
