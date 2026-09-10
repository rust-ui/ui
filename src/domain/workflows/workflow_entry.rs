use crate::__registry__::all_workflows::WorkflowIdKebab;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct WorkflowMeta {
    pub iframe_height: &'static str,
    pub container_class: &'static str,
}

impl WorkflowMeta {
    pub const fn default() -> Self {
        Self { iframe_height: "700px", container_class: "w-full bg-background" }
    }
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct WorkflowEntry {
    pub workflow_id_str: &'static str,
    pub workflow_title: &'static str,
    pub workflow_id_kebab: WorkflowIdKebab,
    pub category: &'static str,
}
