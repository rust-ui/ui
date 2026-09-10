use std::str::FromStr;

use dioxus::prelude::*;

use crate::__registry__::all_blocks::BlockIdKebab;

#[component]
pub fn ViewRouter(id: String) -> Element {
    let Ok(block_id) = BlockIdKebab::from_str(&id) else {
        return rsx! {
            div { class: "flex items-center justify-center h-screen text-sm text-muted-foreground",
                "Block not found: {id}"
            }
        };
    };

    let meta = block_id.meta();

    rsx! {
        section {
            class: "{meta.container_class}",
            "data-name": "__BlockViewPage",
            {block_id.to_component()}
        }
        script {
            r#"
            (function() {{
                if (window.__themeSync) return;
                window.__themeSync = true;
                window.addEventListener('message', function(e) {{
                    if (e.data && e.data.theme) {{
                        if (e.data.theme === 'dark') {{
                            document.documentElement.classList.add('dark');
                            document.body.classList.add('dark');
                        }} else {{
                            document.documentElement.classList.remove('dark');
                            document.body.classList.remove('dark');
                        }}
                    }}
                }});
            }})();
            "#
        }
    }
}
