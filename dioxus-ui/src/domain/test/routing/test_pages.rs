use dioxus::prelude::*;

use crate::domain::test::demos::demo_toolbar::DemoToolbar;
use crate::domain::test::demos::demo_workflow::DemoWorkflow;
use crate::domain::test::demos::demo_workflow_context_menu::DemoWorkflowContextMenu;
use crate::domain::test::demos::demo_workflow_copy_paste::DemoWorkflowCopyPaste;
use crate::domain::test::demos::demo_workflow_export_import::DemoWorkflowExportImport;
use crate::domain::test::demos::demo_workflow_keyboard::DemoWorkflowKeyboard;
use crate::domain::test::demos::demo_workflow_locked_mode::DemoWorkflowLockedMode;
use crate::domain::test::demos::demo_workflow_minimap::DemoWorkflowMinimap;
use crate::domain::test::demos::demo_workflow_multiselect::DemoWorkflowMultiselect;
use crate::domain::test::demos::demo_workflow_status::DemoWorkflowStatus;
use crate::domain::test::demos::demo_workflow_toolbar::DemoWorkflowToolbar;

#[component]
pub fn TestPage() -> Element {
    rsx! {
        div { class: "flex flex-col gap-6",
            div { class: "flex flex-col gap-1",
                h1 { class: "text-2xl font-semibold tracking-tight", "Workflow Canvas" }
                p { class: "text-sm text-muted-foreground", "Drag nodes · Click edge to select, Del to remove · Double-click node to rename · Undo/redo tracks nodes and edges." }
            }
            DemoWorkflow {}

            div { class: "flex flex-col gap-1",
                h2 { class: "text-xl font-semibold tracking-tight", "Workflow — Status" }
                p { class: "text-sm text-muted-foreground", "Node execution status badges. Hit Run / Error / Reset." }
            }
            DemoWorkflowStatus {}

            div { class: "flex flex-col gap-1",
                h2 { class: "text-xl font-semibold tracking-tight", "Workflow — Minimap" }
                p { class: "text-sm text-muted-foreground",
                    "10-node pipeline spread across canvas. Minimap shows full graph at a glance."
                }
            }
            DemoWorkflowMinimap {}

            div { class: "flex flex-col gap-1",
                h2 { class: "text-xl font-semibold tracking-tight", "Workflow — Toolbar" }
                p { class: "text-sm text-muted-foreground",
                    "Toolbar overlay on canvas. Add nodes, undo/redo, reset zoom, delete selected."
                }
            }
            DemoWorkflowToolbar {}

            div { class: "flex flex-col gap-1",
                h2 { class: "text-xl font-semibold tracking-tight", "Workflow — Locked Mode" }
                p { class: "text-sm text-muted-foreground",
                    "Read-only canvas. Toggle lock to freeze all interactions — no drag, pan, zoom, or connect."
                }
            }
            DemoWorkflowLockedMode {}

            div { class: "flex flex-col gap-1",
                h2 { class: "text-xl font-semibold tracking-tight", "Workflow — Multi-select" }
                p { class: "text-sm text-muted-foreground",
                    "Shift+click to add nodes to selection. Shift+drag canvas to rubber-band select. Drag moves all selected. Del removes all."
                }
            }
            DemoWorkflowMultiselect {}

            div { class: "flex flex-col gap-1",
                h2 { class: "text-xl font-semibold tracking-tight", "Workflow — Keyboard" }
                p { class: "text-sm text-muted-foreground",
                    "Select a node, use ↑↓←→ to nudge by grid step (20px). Shift+click for multi-select, then move all at once."
                }
            }
            DemoWorkflowKeyboard {}

            div { class: "flex flex-col gap-1",
                h2 { class: "text-xl font-semibold tracking-tight", "Workflow — Copy / Paste" }
                p { class: "text-sm text-muted-foreground",
                    "Select nodes, Ctrl+C to copy, Ctrl+V to paste (offset +20px each time)."
                }
            }
            DemoWorkflowCopyPaste {}

            div { class: "flex flex-col gap-1",
                h2 { class: "text-xl font-semibold tracking-tight", "Workflow — Context Menu" }
                p { class: "text-sm text-muted-foreground",
                    "Right-click a node to rename, duplicate, or delete. Right-click an edge to remove it. Right-click canvas to add a node at cursor, select all, or paste."
                }
            }
            DemoWorkflowContextMenu {}

            div { class: "flex flex-col gap-1",
                h2 { class: "text-xl font-semibold tracking-tight", "Workflow — Export / Import" }
                p { class: "text-sm text-muted-foreground",
                    "↓ Export downloads workflow.json. ↑ Import restores nodes, edges, positions, and viewport."
                }
            }
            DemoWorkflowExportImport {}

            div { class: "flex flex-col gap-1",
                h2 { class: "text-xl font-semibold tracking-tight", "Toolbar" }
                p { class: "text-sm text-muted-foreground",
                    "Composable toolbar with toggle groups, buttons, separators, and links."
                }
            }
            DemoToolbar {}
        }
    }
}
