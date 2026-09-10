use dioxus::prelude::*;
use icons::{Circle, CircleCheck, CircleX, Clock, LoaderCircle};
use registry::hooks::use_workflow::{WorkflowEdge, WorkflowNode, WorkflowNodeKind, use_workflow};
use registry::ui::workflow::{
    WfNode as Node, WfNodeContent as NodeContent, WfNodeDescription as NodeDescription, WfNodeFooter as NodeFooter,
    WfNodeHeader as NodeHeader, WfNodeTitle as NodeTitle, WorkflowCanvas, WorkflowControls, WorkflowNodeWrapper,
};
use tw_merge::tw_merge;

// ── NodeStatus ────────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq, Default)]
pub enum NodeStatus {
    #[default]
    Idle,
    Queued,
    Running,
    Success,
    Failed,
}

impl NodeStatus {
    fn label(&self) -> &'static str {
        match self {
            NodeStatus::Idle => "Idle",
            NodeStatus::Queued => "Queued",
            NodeStatus::Running => "In progress",
            NodeStatus::Success => "Success",
            NodeStatus::Failed => "Failed",
        }
    }

    fn border_class(&self) -> &'static str {
        match self {
            NodeStatus::Idle => "border-l-zinc-200 dark:border-l-zinc-700",
            NodeStatus::Queued => "border-l-amber-400",
            NodeStatus::Running => "border-l-orange-400",
            NodeStatus::Success => "border-l-emerald-500",
            NodeStatus::Failed => "border-l-red-500",
        }
    }

    fn bg_class(&self) -> &'static str {
        match self {
            NodeStatus::Idle => "",
            NodeStatus::Queued => "bg-amber-50/40 dark:bg-amber-950/10",
            NodeStatus::Running => "bg-orange-50/40 dark:bg-orange-950/10",
            NodeStatus::Success => "bg-emerald-50/40 dark:bg-emerald-950/10",
            NodeStatus::Failed => "bg-red-50/40 dark:bg-red-950/10",
        }
    }

    fn icon_class(&self) -> &'static str {
        match self {
            NodeStatus::Idle => "size-3.5 text-zinc-400",
            NodeStatus::Queued => "size-3.5 text-amber-500",
            NodeStatus::Running => "size-3.5 text-orange-500 animate-spin",
            NodeStatus::Success => "size-3.5 text-emerald-500",
            NodeStatus::Failed => "size-3.5 text-red-500",
        }
    }

    fn label_class(&self) -> &'static str {
        match self {
            NodeStatus::Idle => "text-[11px] font-medium text-zinc-400",
            NodeStatus::Queued => "text-[11px] font-medium text-amber-600 dark:text-amber-400",
            NodeStatus::Running => "text-[11px] font-medium text-orange-600 dark:text-orange-400",
            NodeStatus::Success => "text-[11px] font-medium text-emerald-600 dark:text-emerald-400",
            NodeStatus::Failed => "text-[11px] font-medium text-red-600 dark:text-red-400",
        }
    }
}

// ── Status icon ───────────────────────────────────────────────────────────────

#[component]
fn StatusIcon(status: NodeStatus) -> Element {
    let cls = status.icon_class();
    match status {
        NodeStatus::Idle => rsx! { Circle { class: cls } },
        NodeStatus::Queued => rsx! { Clock { class: cls } },
        NodeStatus::Running => rsx! { LoaderCircle { class: cls } },
        NodeStatus::Success => rsx! { CircleCheck { class: cls } },
        NodeStatus::Failed => rsx! { CircleX { class: cls } },
    }
}

// ── StatusNodeContent ─────────────────────────────────────────────────────────

#[component]
fn StatusNodeContent(node: WorkflowNode, status: NodeStatus) -> Element {
    let node_class = tw_merge!("border-l-[3px]", status.border_class(), status.bg_class(),);
    rsx! {
        Node {
            target: false,
            source: false,
            class: node_class,
            NodeHeader {
                span { class: tw_merge!("size-2 rounded-full shrink-0", node.kind.dot_color()) }
                NodeTitle { class: node.kind.text_color(), "{node.label}" }
                span {
                    class: "ml-auto text-[10px] text-muted-foreground uppercase tracking-wide shrink-0",
                    "{node.kind.label()}"
                }
            }
            NodeContent {
                NodeDescription { class: "font-mono", "{node.description}" }
            }
            NodeFooter {
                class: "py-1.5",
                StatusIcon { status: status.clone() }
                span { class: status.label_class(), "{status.label()}" }
            }
        }
    }
}

// ── Demo ──────────────────────────────────────────────────────────────────────

fn initial_nodes() -> Vec<WorkflowNode> {
    vec![
        WorkflowNode {
            id: "trigger".to_string(),
            initial_x: 32.0,
            initial_y: 150.0,
            width: 200.0,
            has_target: false,
            has_source: true,
            label: "Webhook".to_string(),
            description: "POST /api/run".to_string(),
            kind: WorkflowNodeKind::Trigger,
        },
        WorkflowNode {
            id: "data".to_string(),
            initial_x: 288.0,
            initial_y: 50.0,
            width: 200.0,
            has_target: true,
            has_source: true,
            label: "Vector DB".to_string(),
            description: "pgvector lookup".to_string(),
            kind: WorkflowNodeKind::Data,
        },
        WorkflowNode {
            id: "agent".to_string(),
            initial_x: 288.0,
            initial_y: 188.0,
            width: 224.0,
            has_target: true,
            has_source: true,
            label: "AI Agent".to_string(),
            description: "claude-sonnet-4-6".to_string(),
            kind: WorkflowNodeKind::Agent,
        },
        WorkflowNode {
            id: "output".to_string(),
            initial_x: 572.0,
            initial_y: 150.0,
            width: 200.0,
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
        WorkflowEdge { from: "trigger".to_string(), to: "agent".to_string(), ..Default::default() },
        WorkflowEdge { from: "data".to_string(), to: "agent".to_string(), ..Default::default() },
        WorkflowEdge { from: "agent".to_string(), to: "output".to_string(), ..Default::default() },
    ]
}

#[component]
pub fn DemoWorkflowStatus() -> Element {
    let state = use_workflow(initial_nodes(), initial_edges());

    rsx! {
        WorkflowCanvas {
            state,
            overlay: rsx! { WorkflowControls { state } },
            for (i, node) in state.nodes.read().iter().cloned().enumerate() {
                WorkflowNodeWrapper { key: "{node.id}", state, idx: i,
                    StatusNodeContent {
                        node: node.clone(),
                        status: match node.id.as_str() {
                            "trigger" => NodeStatus::Success,
                            "data"    => NodeStatus::Failed,
                            "agent"   => NodeStatus::Running,
                            "output"  => NodeStatus::Queued,
                            _         => NodeStatus::Idle,
                        },
                    }
                }
            }
        }
    }
}
