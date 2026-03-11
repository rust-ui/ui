use leptos::prelude::*;

use crate::domain::blocks::block_entry::BlockEntry;
use crate::domain::blocks::components::block_code_panel::BlockCodePanel;
use crate::domain::blocks::components::block_viewer_toolbar::{BlockView, BlockViewerToolbar, ScreenSize};
use crate::domain::markdown_ui::components::resizable_wrapper::ResizableWrapper;

/// Displays a BlockViewer with an iframe.
#[component]
pub fn BlockViewer(block_entry: BlockEntry) -> impl IntoView {
    let view_id_kebab = block_entry.block_id_kebab.to_string();
    let view_id_kebab_clone = view_id_kebab.clone();
    let instance_id = view_id_kebab;

    let screen_size_signal = RwSignal::new(ScreenSize::Desktop);
    let block_view = RwSignal::new(BlockView::Preview);

    let meta = block_entry.block_id_kebab.meta();
    let iframe_height = meta.iframe_height;
    let files = block_entry.block_id_kebab.files();
    let tree = StoredValue::new(block_entry.block_id_kebab.file_tree());

    view! {
        <div data-name="__BlockViewer" id=view_id_kebab_clone class="flex flex-col gap-4 pr-2 md:pr-0 scroll-mt-20">
            <BlockViewerToolbar
                block_entry=block_entry
                screen_size=screen_size_signal
                block_view=block_view
                instance_id=instance_id
            />

            <Show when=move || block_view.get() == BlockView::Preview>
                <ResizableWrapper>
                    <iframe
                        src=block_entry.block_id_kebab.to_full_view_url()
                        class=format!("w-full rounded-lg border-0 h-[{}]", iframe_height)
                        height=iframe_height
                    />
                </ResizableWrapper>
            </Show>

            <Show when=move || block_view.get() == BlockView::Code>
                <BlockCodePanel files=files tree=tree.get_value() />
            </Show>
        </div>

        <script>
            {r#"
            // Iframe theme sync - run only once (v2)
            (function() {
            if (window.iframeThemeSync) return;
            window.iframeThemeSync = true;
            console.log("IFRAME THEME SYNC LOADED!");
            
            function sendThemeToIframes(isDark) {
            const theme = isDark ? 'dark' : 'light';
            document.querySelectorAll('iframe').forEach(iframe => {
            try {
            iframe.contentWindow.postMessage({theme: theme}, '*');
            } catch(e) {
            // Ignore cross-origin errors
            }
            });
            }
            
            // Send initial theme to iframes
            function sendInitialTheme() {
            const isDark = document.body.classList.contains('dark');
            sendThemeToIframes(isDark);
            }
            
            // Send theme when iframes load
            setTimeout(sendInitialTheme, 500);
            setTimeout(sendInitialTheme, 1000);
            
            // Listen for body class changes (from existing theme toggle)
            const observer = new MutationObserver(function(mutations) {
            mutations.forEach(function(mutation) {
            if (mutation.type === 'attributes' && mutation.attributeName === 'class') {
            const isDark = document.body.classList.contains('dark');
            sendThemeToIframes(isDark);
            }
            });
            });
            
            observer.observe(document.body, {
            attributes: true,
            attributeFilter: ['class']
            });
            })();
            
            // Anchor scroll functionality - run only once
            (function() {
            if (window.anchorScrollInit) return;
            window.anchorScrollInit = true;
            
            function scrollToAnchor() {
            const hash = window.location.hash;
            if (hash) {
            const targetId = hash.substring(1);
            const targetElement = document.getElementById(targetId);
            if (targetElement) {
            targetElement.scrollIntoView({
            behavior: 'smooth',
            block: 'start'
            });
            }
            }
            }
            
            // Scroll on page load
            setTimeout(scrollToAnchor, 100);
            setTimeout(scrollToAnchor, 500);
            
            // Scroll when hash changes
            window.addEventListener('hashchange', scrollToAnchor);
            })();
            "#}
        </script>
    }
}
