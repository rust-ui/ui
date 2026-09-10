use dioxus::prelude::*;
use registry::hooks::use_workflow::{WorkflowEdge, WorkflowNode, WorkflowNodeKind, use_workflow};
use registry::ui::workflow::{WorkflowCanvas, WorkflowControls, WorkflowDefaultNode, WorkflowNodeWrapper};

fn initial_nodes() -> Vec<WorkflowNode> {
    vec![
        WorkflowNode {
            id: "trigger".to_string(),
            initial_x: 32.0,
            initial_y: 130.0,
            width: 192.0,
            has_target: false,
            has_source: true,
            label: "User Input".to_string(),
            description: "Starts the workflow".to_string(),
            kind: WorkflowNodeKind::Trigger,
        },
        WorkflowNode {
            id: "data".to_string(),
            initial_x: 272.0,
            initial_y: 40.0,
            width: 192.0,
            has_target: true,
            has_source: true,
            label: "Context".to_string(),
            description: "vectordb".to_string(),
            kind: WorkflowNodeKind::Data,
        },
        WorkflowNode {
            id: "agent".to_string(),
            initial_x: 272.0,
            initial_y: 160.0,
            width: 224.0,
            has_target: true,
            has_source: true,
            label: "AI Agent".to_string(),
            description: "claude-3.5-sonnet".to_string(),
            kind: WorkflowNodeKind::Agent,
        },
        WorkflowNode {
            id: "output".to_string(),
            initial_x: 552.0,
            initial_y: 130.0,
            width: 192.0,
            has_target: true,
            has_source: false,
            label: "Response".to_string(),
            description: "Ready to send".to_string(),
            kind: WorkflowNodeKind::Output,
        },
    ]
}

fn initial_edges() -> Vec<WorkflowEdge> {
    vec![
        WorkflowEdge { from: "trigger".to_string(), to: "agent".to_string(), ..Default::default() },
        WorkflowEdge { from: "data".to_string(), to: "agent".to_string(), ..Default::default() },
        WorkflowEdge { from: "agent".to_string(), to: "output".to_string(), ..Default::default() },
    ]
}

#[component]
pub fn DemoWorkflow() -> Element {
    let state = use_workflow(initial_nodes(), initial_edges());

    rsx! {
        WorkflowCanvas {
            state,
            overlay: rsx! {
                WorkflowControls { state }
            },
            for (i, node) in state.nodes.read().iter().cloned().enumerate() {
                WorkflowNodeWrapper { key: "{node.id}", state, idx: i,
                    WorkflowDefaultNode { node }
                }
            }
        }
    }
}
