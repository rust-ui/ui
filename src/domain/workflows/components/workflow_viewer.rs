use dioxus::prelude::*;

use crate::domain::blocks::components::block_code_panel::BlockCodePanel;
use crate::domain::blocks::components::block_viewer_toolbar::{BlockView, ScreenSize};
use crate::domain::markdown_ui::components::resizable_wrapper::ResizableWrapper;
use crate::domain::workflows::components::workflow_viewer_toolbar::WorkflowViewerToolbar;
use crate::domain::workflows::workflow_entry::WorkflowEntry;

#[component]
pub fn WorkflowViewer(workflow_entry: WorkflowEntry) -> Element {
    let workflow_id = workflow_entry.workflow_id_kebab;
    let workflow_id_str = workflow_entry.workflow_id_str;
    let meta = workflow_id.meta();
    let iframe_height = meta.iframe_height;
    let instance_id = workflow_id_str.to_string();

    let block_view = use_signal(BlockView::default);
    let screen_size = use_signal(ScreenSize::default);

    let files = workflow_id.files();
    let tree = use_hook(|| workflow_id.file_tree());

    rsx! {
        div {
            id: workflow_id_str,
            "data-name": "__WorkflowViewer",
            class: "flex flex-col gap-4 pr-2 md:pr-0 scroll-mt-20",

            WorkflowViewerToolbar {
                workflow_entry,
                screen_size,
                block_view,
                instance_id: instance_id.clone(),
            }

            if block_view() == BlockView::Preview {
                ResizableWrapper { instance_id,
                    iframe {
                        src: workflow_id.to_full_view_url(),
                        class: "w-full rounded-lg border-0",
                        height: "{iframe_height}",
                        style: "height: {iframe_height};",
                    }
                }
            }

            if block_view() == BlockView::Code {
                BlockCodePanel { files, tree }
            }
        }

        script {
            r#"
            (function() {{
                if (window.__iframeThemeSync) return;
                window.__iframeThemeSync = true;

                function sendThemeToIframes(isDark) {{
                    var theme = isDark ? 'dark' : 'light';
                    document.querySelectorAll('iframe').forEach(function(iframe) {{
                        try {{ iframe.contentWindow.postMessage({{ theme: theme }}, '*'); }} catch(e) {{}}
                    }});
                }}

                function sendInitialTheme() {{
                    sendThemeToIframes(document.documentElement.classList.contains('dark'));
                }}

                setTimeout(sendInitialTheme, 500);
                setTimeout(sendInitialTheme, 1000);

                var observer = new MutationObserver(function(mutations) {{
                    mutations.forEach(function(m) {{
                        if (m.type === 'attributes' && m.attributeName === 'class') {{
                            sendThemeToIframes(document.documentElement.classList.contains('dark'));
                        }}
                    }});
                }});
                observer.observe(document.documentElement, {{ attributes: true, attributeFilter: ['class'] }});
            }})();

            (function() {{
                if (window.__anchorScrollInit) return;
                window.__anchorScrollInit = true;

                function scrollToAnchor() {{
                    var hash = window.location.hash;
                    if (hash) {{
                        var el = document.getElementById(hash.substring(1));
                        if (el) el.scrollIntoView({{ behavior: 'smooth', block: 'start' }});
                    }}
                }}

                setTimeout(scrollToAnchor, 100);
                setTimeout(scrollToAnchor, 500);
                window.addEventListener('hashchange', scrollToAnchor);
            }})();
            "#
        }
    }
}
