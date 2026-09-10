use dioxus::prelude::*;
use icons::{Clipboard, ClipboardCopy, Trash2};

use crate::hooks::use_workflow::{WorkflowEdge, WorkflowNode, WorkflowNodeKind, use_workflow};
use crate::ui::toolbar::{Toolbar, ToolbarButton, ToolbarSeparator};
use crate::ui::workflow::{WorkflowCanvas, WorkflowControls, WorkflowDefaultNode, WorkflowNodeWrapper};

fn initial_nodes() -> Vec<WorkflowNode> {
    vec![
        WorkflowNode {
            id: "a".to_string(),
            initial_x: 60.0,
            initial_y: 100.0,
            width: 192.0,
            has_target: false,
            has_source: true,
            label: "Input".to_string(),
            description: "Entry point".to_string(),
            kind: WorkflowNodeKind::Trigger,
        },
        WorkflowNode {
            id: "b".to_string(),
            initial_x: 300.0,
            initial_y: 60.0,
            width: 192.0,
            has_target: true,
            has_source: true,
            label: "Processor".to_string(),
            description: "Transform data".to_string(),
            kind: WorkflowNodeKind::Agent,
        },
        WorkflowNode {
            id: "c".to_string(),
            initial_x: 300.0,
            initial_y: 200.0,
            width: 192.0,
            has_target: true,
            has_source: true,
            label: "Validator".to_string(),
            description: "Check output".to_string(),
            kind: WorkflowNodeKind::Data,
        },
        WorkflowNode {
            id: "d".to_string(),
            initial_x: 540.0,
            initial_y: 100.0,
            width: 192.0,
            has_target: true,
            has_source: false,
            label: "Output".to_string(),
            description: "Final result".to_string(),
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
    ]
}

#[component]
pub fn Workflow02() -> Element {
    let mut state = use_workflow(initial_nodes(), initial_edges());
    let selected_count = state.selected.read().len();
    let clipboard_count = state.clipboard_count();

    rsx! {
        WorkflowCanvas {
            state,
            overlay: rsx! {
                div {
                    class: "absolute top-3 left-1/2 -translate-x-1/2 z-20",
                    Toolbar { aria_label: "Copy paste controls",
                        ToolbarButton {
                            onclick: move |_| state.copy_selected(),
                            disabled: selected_count == 0,
                            ClipboardCopy {}
                            "Copy"
                            if selected_count > 0 {
                                span { class: "ml-1 text-xs text-muted-foreground", "({selected_count})" }
                            }
                        }
                        ToolbarButton {
                            onclick: move |_| state.paste_nodes(),
                            disabled: clipboard_count == 0,
                            Clipboard {}
                            "Paste"
                            if clipboard_count > 0 {
                                span { class: "ml-1 text-xs text-muted-foreground", "({clipboard_count})" }
                            }
                        }
                        ToolbarSeparator {}
                        ToolbarButton {
                            onclick: move |_| state.delete_selected(),
                            disabled: selected_count == 0,
                            Trash2 {}
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
