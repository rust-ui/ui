use dioxus::prelude::*;
use registry::hooks::use_workflow::{WorkflowEdge, WorkflowNode, WorkflowNodeKind, WorkflowSnapshot, use_workflow};
use registry::ui::workflow::{WorkflowCanvas, WorkflowControls, WorkflowDefaultNode, WorkflowNodeWrapper};

fn initial_nodes() -> Vec<WorkflowNode> {
    vec![
        WorkflowNode {
            id: "input".to_string(),
            initial_x: 40.0,
            initial_y: 100.0,
            width: 192.0,
            has_target: false,
            has_source: true,
            label: "Input".to_string(),
            description: "Data source".to_string(),
            kind: WorkflowNodeKind::Data,
        },
        WorkflowNode {
            id: "proc".to_string(),
            initial_x: 290.0,
            initial_y: 60.0,
            width: 192.0,
            has_target: true,
            has_source: true,
            label: "Processor".to_string(),
            description: "Transform data".to_string(),
            kind: WorkflowNodeKind::Agent,
        },
        WorkflowNode {
            id: "val".to_string(),
            initial_x: 290.0,
            initial_y: 170.0,
            width: 192.0,
            has_target: true,
            has_source: true,
            label: "Validator".to_string(),
            description: "Check rules".to_string(),
            kind: WorkflowNodeKind::Agent,
        },
        WorkflowNode {
            id: "out".to_string(),
            initial_x: 540.0,
            initial_y: 115.0,
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
        WorkflowEdge { from: "input".to_string(), to: "proc".to_string(), ..Default::default() },
        WorkflowEdge { from: "input".to_string(), to: "val".to_string(), ..Default::default() },
        WorkflowEdge { from: "proc".to_string(), to: "out".to_string(), ..Default::default() },
        WorkflowEdge { from: "val".to_string(), to: "out".to_string(), ..Default::default() },
    ]
}

#[component]
pub fn DemoWorkflowExportImport() -> Element {
    let state = use_workflow(initial_nodes(), initial_edges());
    let mut state = state;
    let mut import_error = use_signal(|| Option::<String>::None);

    let overlay = rsx! {
        div {
            class: "flex items-center gap-1 rounded-md border bg-background/90 backdrop-blur-sm shadow-sm px-1.5 py-1",
            style: "position: absolute; top: 12px; left: 12px;",

            button {
                class: "text-[11px] px-2 py-0.5 rounded hover:bg-accent text-muted-foreground transition-colors flex items-center gap-1",
                title: "Export canvas as JSON",
                onclick: move |_| {
                    #[cfg(target_arch = "wasm32")]
                    {
                        let snap = state.export_snapshot();
                        let Ok(json) = serde_json::to_string_pretty(&snap) else { return };
                        use wasm_bindgen::JsCast;
                        let arr = js_sys::Array::new();
                        arr.push(&wasm_bindgen::JsValue::from_str(&json));
                        let mut opts = web_sys::BlobPropertyBag::new();
                        opts.type_("application/json");
                        if let Ok(blob) = web_sys::Blob::new_with_str_sequence_and_options(&arr, &opts) {
                            if let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) {
                                if let Some(window) = web_sys::window() {
                                    if let Some(document) = window.document() {
                                        if let Ok(el) = document.create_element("a") {
                                            if let Ok(a) = el.dyn_into::<web_sys::HtmlAnchorElement>() {
                                                a.set_href(&url);
                                                a.set_download("workflow.json");
                                                if let Some(body) = document.body() {
                                                    let _ = body.append_child(&a);
                                                    a.click();
                                                    let _ = body.remove_child(&a);
                                                }
                                            }
                                        }
                                        let _ = web_sys::Url::revoke_object_url(&url);
                                    }
                                }
                            }
                        }
                    }
                },
                "↓ Export"
            }

            label {
                class: "text-[11px] px-2 py-0.5 rounded hover:bg-accent text-muted-foreground transition-colors cursor-pointer",
                title: "Import canvas from JSON",
                r#for: "wf-import-input",
                "↑ Import"
            }
            input {
                id: "wf-import-input",
                r#type: "file",
                accept: ".json",
                class: "hidden",
                onchange: move |ev| {
                    import_error.set(None);
                    let files = ev.files();
                    let Some(file) = files.into_iter().next() else { return };
                    spawn(async move {
                        let content = match file.read_string().await {
                            Ok(s) => s,
                            Err(e) => { import_error.set(Some(format!("Read error: {e}"))); return; }
                        };
                        match serde_json::from_str::<WorkflowSnapshot>(&content) {
                            Ok(snap) => state.load_snapshot(snap),
                            Err(e) => import_error.set(Some(format!("Invalid JSON: {e}"))),
                        }
                    });
                },
            }

            if let Some(err) = import_error.read().clone() {
                span { class: "text-[10px] text-destructive ml-1", "{err}" }
            }
        }

        WorkflowControls { state }
    };

    rsx! {
        WorkflowCanvas {
            state,
            overlay,
            for (i, node) in state.nodes.read().iter().cloned().enumerate() {
                WorkflowNodeWrapper { key: "{node.id}", state, idx: i,
                    WorkflowDefaultNode { node }
                }
            }
        }
    }
}
