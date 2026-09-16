use dioxus::prelude::*;
use icons::{MousePointerClick, Trash2};

use crate::hooks::use_workflow::{WorkflowEdge, WorkflowNode, WorkflowNodeKind, use_workflow};
use crate::ui::toolbar::{Toolbar, ToolbarButton, ToolbarSeparator};
use crate::ui::workflow::{WorkflowCanvas, WorkflowControls, WorkflowDefaultNode, WorkflowNodeWrapper};

fn initial_nodes() -> Vec<WorkflowNode> {
    vec![
        WorkflowNode {
            id: "a".to_string(),
            initial_x: 40.0,
            initial_y: 80.0,
            width: 192.0,
            has_target: false,
            has_source: true,
            label: "Trigger".to_string(),
            description: "Entry point".to_string(),
            kind: WorkflowNodeKind::Trigger,
        },
        WorkflowNode {
            id: "b".to_string(),
            initial_x: 280.0,
            initial_y: 40.0,
            width: 192.0,
            has_target: true,
            has_source: true,
            label: "Fetch Data".to_string(),
            description: "HTTP request".to_string(),
            kind: WorkflowNodeKind::Data,
        },
        WorkflowNode {
            id: "c".to_string(),
            initial_x: 280.0,
            initial_y: 180.0,
            width: 192.0,
            has_target: true,
            has_source: true,
            label: "Cache".to_string(),
            description: "Redis lookup".to_string(),
            kind: WorkflowNodeKind::Data,
        },
        WorkflowNode {
            id: "d".to_string(),
            initial_x: 520.0,
            initial_y: 80.0,
            width: 224.0,
            has_target: true,
            has_source: true,
            label: "AI Agent".to_string(),
            description: "claude-sonnet".to_string(),
            kind: WorkflowNodeKind::Agent,
        },
        WorkflowNode {
            id: "e".to_string(),
            initial_x: 520.0,
            initial_y: 240.0,
            width: 192.0,
            has_target: true,
            has_source: true,
            label: "Logger".to_string(),
            description: "Audit trail".to_string(),
            kind: WorkflowNodeKind::Data,
        },
        WorkflowNode {
            id: "f".to_string(),
            initial_x: 780.0,
            initial_y: 80.0,
            width: 192.0,
            has_target: true,
            has_source: false,
            label: "Response".to_string(),
            description: "Return result".to_string(),
            kind: WorkflowNodeKind::Output,
        },
    ]
}

fn initial_edges() -> Vec<WorkflowEdge> {
    vec![
        WorkflowEdge { from: "a".to_string(), to: "b".to_string(), ..Default::default() },
        WorkflowEdge { from: "a".to_string(), to: "c".to_string(), ..Default::default() },
        WorkflowEdge { from: "b".to_string(), to: "d".to_string(), ..Default::default() },
        WorkflowEdge { from: "c".to_string(), to: "d".to_string(), ..Default::default() },
        WorkflowEdge { from: "d".to_string(), to: "e".to_string(), ..Default::default() },
        WorkflowEdge { from: "d".to_string(), to: "f".to_string(), ..Default::default() },
    ]
}

#[component]
pub fn Workflow06() -> Element {
    let mut state = use_workflow(initial_nodes(), initial_edges());
    let selected_count = state.selected.read().len();

    rsx! {
        WorkflowCanvas {
            state,
            overlay: rsx! {
                div {
                    class: "absolute top-3 left-1/2 -translate-x-1/2 z-20",
                    Toolbar { aria_label: "Multi-select controls",
                        if selected_count > 0 {
                            span {
                                class: "flex items-center gap-1 px-2 text-xs font-medium text-primary",
                                MousePointerClick {}
                                "{selected_count} selected"
                            }
                            ToolbarSeparator {}
                        }
                        ToolbarButton {
                            onclick: move |_| state.delete_selected(),
                            disabled: selected_count == 0,
                            Trash2 {}
                            "Delete selected"
                        }
                    }
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
