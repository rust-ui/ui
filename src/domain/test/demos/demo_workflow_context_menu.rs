use dioxus::prelude::*;
use registry::hooks::use_workflow::{EdgeStyle, WorkflowEdge, WorkflowNode, WorkflowNodeKind, use_workflow};
use registry::ui::workflow::{WorkflowCanvas, WorkflowControls, WorkflowDefaultNode, WorkflowNodeWrapper};

fn initial_nodes() -> Vec<WorkflowNode> {
    vec![
        WorkflowNode {
            id: "a".to_string(),
            initial_x: 40.0,
            initial_y: 120.0,
            width: 192.0,
            has_target: false,
            has_source: true,
            label: "Webhook".to_string(),
            description: "Incoming trigger".to_string(),
            kind: WorkflowNodeKind::Trigger,
        },
        WorkflowNode {
            id: "b".to_string(),
            initial_x: 290.0,
            initial_y: 60.0,
            width: 192.0,
            has_target: true,
            has_source: true,
            label: "Parse JSON".to_string(),
            description: "Extract fields".to_string(),
            kind: WorkflowNodeKind::Data,
        },
        WorkflowNode {
            id: "c".to_string(),
            initial_x: 290.0,
            initial_y: 220.0,
            width: 192.0,
            has_target: true,
            has_source: true,
            label: "Validate".to_string(),
            description: "Schema check".to_string(),
            kind: WorkflowNodeKind::Data,
        },
        WorkflowNode {
            id: "d".to_string(),
            initial_x: 540.0,
            initial_y: 120.0,
            width: 224.0,
            has_target: true,
            has_source: true,
            label: "AI Processor".to_string(),
            description: "claude-sonnet".to_string(),
            kind: WorkflowNodeKind::Agent,
        },
        WorkflowNode {
            id: "e".to_string(),
            initial_x: 820.0,
            initial_y: 120.0,
            width: 192.0,
            has_target: true,
            has_source: false,
            label: "Send Result".to_string(),
            description: "HTTP response".to_string(),
            kind: WorkflowNodeKind::Output,
        },
    ]
}

fn initial_edges() -> Vec<WorkflowEdge> {
    vec![
        WorkflowEdge { from: "a".to_string(), to: "b".to_string(), style: EdgeStyle::Dashed, label: None },
        WorkflowEdge { from: "a".to_string(), to: "c".to_string(), style: EdgeStyle::Dotted, label: None },
        WorkflowEdge { from: "b".to_string(), to: "d".to_string(), style: EdgeStyle::Solid, label: None },
        WorkflowEdge { from: "c".to_string(), to: "d".to_string(), style: EdgeStyle::Solid, label: None },
        WorkflowEdge { from: "d".to_string(), to: "e".to_string(), style: EdgeStyle::Solid, label: None },
    ]
}

#[component]
pub fn DemoWorkflowContextMenu() -> Element {
    let state = use_workflow(initial_nodes(), initial_edges());

    rsx! {
        WorkflowCanvas {
            state,
            overlay: rsx! {
                div {
                    class: "absolute top-3 left-3 z-20 rounded-md border bg-card/90 backdrop-blur-sm shadow-sm px-3 py-2 text-xs text-muted-foreground space-y-0.5",
                    p { class: "font-medium text-foreground mb-1", "Right-click menus" }
                    p { "🖱 Node → Rename · Duplicate · Delete" }
                    p { "🖱 Edge → Delete Edge" }
                    p { "🖱 Canvas → Add Node Here · Select All" }
                }
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
