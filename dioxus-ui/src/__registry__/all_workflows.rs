use dioxus::prelude::*;

use crate::domain::blocks::block_entry::{BlockFile, BlockFileTreeItem};
use crate::domain::workflows::workflow_entry::{WorkflowEntry, WorkflowMeta};

// ─── Enum ─────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum WorkflowIdKebab {
    Workflow01,
    Workflow02,
    Workflow03,
    Workflow04,
    Workflow05,
    Workflow06,
    Workflow07,
    Workflow08,
}

// ─── Display (kebab-case) ─────────────────────────────────────────────────────

impl std::fmt::Display for WorkflowIdKebab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Workflow01 => "workflow-01",
            Self::Workflow02 => "workflow-02",
            Self::Workflow03 => "workflow-03",
            Self::Workflow04 => "workflow-04",
            Self::Workflow05 => "workflow-05",
            Self::Workflow06 => "workflow-06",
            Self::Workflow07 => "workflow-07",
            Self::Workflow08 => "workflow-08",
        };
        write!(f, "{s}")
    }
}

// ─── from_kebab ───────────────────────────────────────────────────────────────

impl WorkflowIdKebab {
    pub fn from_kebab(s: &str) -> Option<Self> {
        match s {
            "workflow-01" => Some(Self::Workflow01),
            "workflow-02" => Some(Self::Workflow02),
            "workflow-03" => Some(Self::Workflow03),
            "workflow-04" => Some(Self::Workflow04),
            "workflow-05" => Some(Self::Workflow05),
            "workflow-06" => Some(Self::Workflow06),
            "workflow-07" => Some(Self::Workflow07),
            "workflow-08" => Some(Self::Workflow08),
            _ => None,
        }
    }

    // ─── Title ───────────────────────────────────────────────────────────────

    pub fn to_title(&self) -> &'static str {
        match self {
            Self::Workflow01 => "Basic Workflow",
            Self::Workflow02 => "Copy & Paste",
            Self::Workflow03 => "Keyboard Shortcuts",
            Self::Workflow04 => "Locked Mode",
            Self::Workflow05 => "Minimap",
            Self::Workflow06 => "Multi-Select",
            Self::Workflow07 => "Status Nodes",
            Self::Workflow08 => "Toolbar",
        }
    }

    // ─── Meta ─────────────────────────────────────────────────────────────────

    pub fn meta(&self) -> WorkflowMeta {
        WorkflowMeta::default()
    }

    // ─── to_full_view_url ─────────────────────────────────────────────────────

    pub fn to_full_view_url(&self) -> String {
        format!("/view/{}", self)
    }

    // ─── files ────────────────────────────────────────────────────────────────

    pub fn files(&self) -> &'static [BlockFile] {
        match self {
            Self::Workflow01 => &[BlockFile {
                name: "workflow01.rs",
                target: "workflows/workflow01.rs",
                content: include_str!("../../app_crates/registry/src/workflows/workflow01.rs"),
                language: "rust",
            }],
            Self::Workflow02 => &[BlockFile {
                name: "workflow02.rs",
                target: "workflows/workflow02.rs",
                content: include_str!("../../app_crates/registry/src/workflows/workflow02.rs"),
                language: "rust",
            }],
            Self::Workflow03 => &[BlockFile {
                name: "workflow03.rs",
                target: "workflows/workflow03.rs",
                content: include_str!("../../app_crates/registry/src/workflows/workflow03.rs"),
                language: "rust",
            }],
            Self::Workflow04 => &[BlockFile {
                name: "workflow04.rs",
                target: "workflows/workflow04.rs",
                content: include_str!("../../app_crates/registry/src/workflows/workflow04.rs"),
                language: "rust",
            }],
            Self::Workflow05 => &[BlockFile {
                name: "workflow05.rs",
                target: "workflows/workflow05.rs",
                content: include_str!("../../app_crates/registry/src/workflows/workflow05.rs"),
                language: "rust",
            }],
            Self::Workflow06 => &[BlockFile {
                name: "workflow06.rs",
                target: "workflows/workflow06.rs",
                content: include_str!("../../app_crates/registry/src/workflows/workflow06.rs"),
                language: "rust",
            }],
            Self::Workflow07 => &[BlockFile {
                name: "workflow07.rs",
                target: "workflows/workflow07.rs",
                content: include_str!("../../app_crates/registry/src/workflows/workflow07.rs"),
                language: "rust",
            }],
            Self::Workflow08 => &[BlockFile {
                name: "workflow08.rs",
                target: "workflows/workflow08.rs",
                content: include_str!("../../app_crates/registry/src/workflows/workflow08.rs"),
                language: "rust",
            }],
        }
    }

    // ─── file_tree ────────────────────────────────────────────────────────────

    pub fn file_tree(&self) -> Vec<BlockFileTreeItem> {
        let file_name = match self {
            Self::Workflow01 => "workflow01.rs",
            Self::Workflow02 => "workflow02.rs",
            Self::Workflow03 => "workflow03.rs",
            Self::Workflow04 => "workflow04.rs",
            Self::Workflow05 => "workflow05.rs",
            Self::Workflow06 => "workflow06.rs",
            Self::Workflow07 => "workflow07.rs",
            Self::Workflow08 => "workflow08.rs",
        };
        vec![BlockFileTreeItem::Folder {
            name: "workflows",
            items: vec![BlockFileTreeItem::File { name: file_name, index: 0 }],
        }]
    }

    // ─── to_component ─────────────────────────────────────────────────────────

    pub fn to_component(&self) -> Element {
        match self {
            Self::Workflow01 => registry::workflows::workflow01::Workflow01(),
            Self::Workflow02 => registry::workflows::workflow02::Workflow02(),
            Self::Workflow03 => registry::workflows::workflow03::Workflow03(),
            Self::Workflow04 => registry::workflows::workflow04::Workflow04(),
            Self::Workflow05 => registry::workflows::workflow05::Workflow05(),
            Self::Workflow06 => registry::workflows::workflow06::Workflow06(),
            Self::Workflow07 => registry::workflows::workflow07::Workflow07(),
            Self::Workflow08 => registry::workflows::workflow08::Workflow08(),
        }
    }
}

// ─── Static arrays ────────────────────────────────────────────────────────────

pub const ALL_WORKFLOWS: &[WorkflowEntry] = &[
    WorkflowEntry {
        workflow_id_str: "workflow-01",
        workflow_title: "Basic Workflow",
        workflow_id_kebab: WorkflowIdKebab::Workflow01,
        category: "workflows",
    },
    WorkflowEntry {
        workflow_id_str: "workflow-02",
        workflow_title: "Copy & Paste",
        workflow_id_kebab: WorkflowIdKebab::Workflow02,
        category: "workflows",
    },
    WorkflowEntry {
        workflow_id_str: "workflow-03",
        workflow_title: "Keyboard Shortcuts",
        workflow_id_kebab: WorkflowIdKebab::Workflow03,
        category: "workflows",
    },
    WorkflowEntry {
        workflow_id_str: "workflow-04",
        workflow_title: "Locked Mode",
        workflow_id_kebab: WorkflowIdKebab::Workflow04,
        category: "workflows",
    },
    WorkflowEntry {
        workflow_id_str: "workflow-05",
        workflow_title: "Minimap",
        workflow_id_kebab: WorkflowIdKebab::Workflow05,
        category: "workflows",
    },
    WorkflowEntry {
        workflow_id_str: "workflow-06",
        workflow_title: "Multi-Select",
        workflow_id_kebab: WorkflowIdKebab::Workflow06,
        category: "workflows",
    },
    WorkflowEntry {
        workflow_id_str: "workflow-07",
        workflow_title: "Status Nodes",
        workflow_id_kebab: WorkflowIdKebab::Workflow07,
        category: "workflows",
    },
    WorkflowEntry {
        workflow_id_str: "workflow-08",
        workflow_title: "Toolbar",
        workflow_id_kebab: WorkflowIdKebab::Workflow08,
        category: "workflows",
    },
];
