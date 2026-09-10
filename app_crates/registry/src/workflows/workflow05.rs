use dioxus::prelude::*;

use crate::hooks::use_workflow::{WorkflowEdge, WorkflowNode, WorkflowNodeKind, use_workflow};
use crate::ui::workflow::{
    WorkflowCanvas, WorkflowControls, WorkflowDefaultNode, WorkflowMinimap, WorkflowNodeWrapper,
};

fn initial_nodes() -> Vec<WorkflowNode> {
    vec![
        WorkflowNode {
            id: "input".to_string(),
            initial_x: 40.0,
            initial_y: 300.0,
            width: 192.0,
            has_target: false,
            has_source: true,
            label: "User Input".to_string(),
            description: "Entry point".to_string(),
            kind: WorkflowNodeKind::Trigger,
        },
        WorkflowNode {
            id: "auth".to_string(),
            initial_x: 300.0,
            initial_y: 160.0,
            width: 192.0,
            has_target: true,
            has_source: true,
            label: "Auth Check".to_string(),
            description: "Validate token".to_string(),
            kind: WorkflowNodeKind::Data,
        },
        WorkflowNode {
            id: "cache".to_string(),
            initial_x: 300.0,
            initial_y: 300.0,
            width: 192.0,
            has_target: true,
            has_source: true,
            label: "Cache Lookup".to_string(),
            description: "Redis cache".to_string(),
            kind: WorkflowNodeKind::Data,
        },
        WorkflowNode {
            id: "retrieval".to_string(),
            initial_x: 300.0,
            initial_y: 440.0,
            width: 192.0,
            has_target: true,
            has_source: true,
            label: "Vector Search".to_string(),
            description: "pgvector".to_string(),
            kind: WorkflowNodeKind::Data,
        },
        WorkflowNode {
            id: "rerank".to_string(),
            initial_x: 560.0,
            initial_y: 370.0,
            width: 192.0,
            has_target: true,
            has_source: true,
            label: "Re-ranker".to_string(),
            description: "cross-encoder".to_string(),
            kind: WorkflowNodeKind::Data,
        },
        WorkflowNode {
            id: "agent".to_string(),
            initial_x: 820.0,
            initial_y: 260.0,
            width: 224.0,
            has_target: true,
            has_source: true,
            label: "AI Agent".to_string(),
            description: "claude-sonnet-4-6".to_string(),
            kind: WorkflowNodeKind::Agent,
        },
        WorkflowNode {
            id: "guard".to_string(),
            initial_x: 820.0,
            initial_y: 420.0,
            width: 192.0,
            has_target: true,
            has_source: true,
            label: "Safety Guard".to_string(),
            description: "content filter".to_string(),
            kind: WorkflowNodeKind::Data,
        },
        WorkflowNode {
            id: "formatter".to_string(),
            initial_x: 1100.0,
            initial_y: 300.0,
            width: 192.0,
            has_target: true,
            has_source: true,
            label: "Formatter".to_string(),
            description: "markdown → html".to_string(),
            kind: WorkflowNodeKind::Data,
        },
        WorkflowNode {
            id: "logger".to_string(),
            initial_x: 1100.0,
            initial_y: 460.0,
            width: 192.0,
            has_target: true,
            has_source: false,
            label: "Logger".to_string(),
            description: "audit trail".to_string(),
            kind: WorkflowNodeKind::Output,
        },
        WorkflowNode {
            id: "output".to_string(),
            initial_x: 1360.0,
            initial_y: 300.0,
            width: 192.0,
            has_target: true,
            has_source: false,
            label: "Response".to_string(),
            description: "stream to client".to_string(),
            kind: WorkflowNodeKind::Output,
        },
    ]
}

fn initial_edges() -> Vec<WorkflowEdge> {
    vec![
        WorkflowEdge { from: "input".to_string(), to: "auth".to_string(), ..Default::default() },
        WorkflowEdge { from: "input".to_string(), to: "cache".to_string(), ..Default::default() },
        WorkflowEdge { from: "input".to_string(), to: "retrieval".to_string(), ..Default::default() },
        WorkflowEdge { from: "retrieval".to_string(), to: "rerank".to_string(), ..Default::default() },
        WorkflowEdge { from: "cache".to_string(), to: "agent".to_string(), ..Default::default() },
        WorkflowEdge { from: "auth".to_string(), to: "agent".to_string(), ..Default::default() },
        WorkflowEdge { from: "rerank".to_string(), to: "agent".to_string(), ..Default::default() },
        WorkflowEdge { from: "agent".to_string(), to: "guard".to_string(), ..Default::default() },
        WorkflowEdge { from: "agent".to_string(), to: "formatter".to_string(), ..Default::default() },
        WorkflowEdge { from: "guard".to_string(), to: "logger".to_string(), ..Default::default() },
        WorkflowEdge { from: "formatter".to_string(), to: "output".to_string(), ..Default::default() },
    ]
}

#[component]
pub fn Workflow05() -> Element {
    let state = use_workflow(initial_nodes(), initial_edges());

    rsx! {
        WorkflowCanvas {
            state,
            overlay: rsx! {
                WorkflowControls { state }
                WorkflowMinimap { state }
            },
            for (i, node) in state.nodes.read().iter().cloned().enumerate() {
                WorkflowNodeWrapper { key: "{node.id}", state, idx: i,
                    WorkflowDefaultNode { node }
                }
            }
        }
    }
}
