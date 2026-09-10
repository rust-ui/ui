use dioxus::prelude::*;

use crate::__registry__::all_workflows::WorkflowIdKebab;

#[component]
pub fn WorkflowViewPage(id: String) -> Element {
    let Some(wf) = WorkflowIdKebab::from_kebab(&id) else {
        return rsx! {
            div { class: "flex items-center justify-center h-screen text-sm text-muted-foreground",
                "Workflow not found: {id}"
            }
        };
    };

    rsx! {
        div { class: "h-screen w-screen overflow-hidden p-4 bg-background",
            {wf.to_component()}
        }

        script {
            r#"
            window.addEventListener('message', function(e) {{
                if (e.data && e.data.theme) {{
                    document.documentElement.classList.toggle('dark', e.data.theme === 'dark');
                    document.documentElement.classList.toggle('light', e.data.theme === 'light');
                }}
            }});
            "#
        }
    }
}
