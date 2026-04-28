use leptos::prelude::*;

use crate::__registry__::static_md_registry::MyMd;
use crate::components::table_of_contents::{TocItem, extract_toc_from_md};
use crate::domain::markdown_ui::api_read_md::read_md_file_typed;
use crate::domain::markdown_ui::components::md_skeleton::MdSkeletonDemo;

#[component]
pub fn SharedDemoMdWithToc(
    md_path: &'static str,
    #[prop(into)] toc_signal: ArcWriteSignal<Vec<TocItem>>,
) -> impl IntoView {
    let md_file_resource = Resource::new(|| (), move |_| async move { read_md_file_typed(md_path.to_string()).await });

    view! {
        // * Using Transition instead of Suspense here because each sidenav navigation unmounts/remounts
        // * SharedRoutesDemo, creating a new Resource that fires a server fn call (/api/ReadMdFileTyped).
        // * On production, this causes a visible skeleton flash that looks like a full page reload.
        // * Transition keeps the previous page content visible while the new page loads, eliminating the flash.
        <Transition fallback=move || {
            view! { <MdSkeletonDemo /> }
        }>
            {move || {
                md_file_resource
                    .get()
                    .map(|result| match result {
                        Ok(md_file) => {
                            let toc_items = extract_toc_from_md(&md_file);
                            toc_signal.set(toc_items);
                            view! { <MyMd md_file=md_file /> }.into_any()
                        }
                        Err(err) => {
                            view! {
                                <div class="p-4 text-red-500">
                                    <h3 class="font-bold">"Error loading content"</h3>
                                    <p>{err.to_string()}</p>
                                </div>
                            }
                                .into_any()
                        }
                    })
            }}
        </Transition>
    }
}
