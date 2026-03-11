use std::sync::Arc;

use app_domain::markdown_config::RegistryEntry;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn SharedSidenavDemos(
    base_path: Arc<String>,
    params_demo_name: Arc<impl Fn() -> String + Send + Sync + 'static>,
    all_arc_demos: Vec<(&'static str, Arc<Vec<RegistryEntry>>)>,
) -> impl IntoView {
    // * 💪🐛 Using Memo solved the issue of active class not updating when the pathname changes.
    // * Memo is a utility that allows you to create a reactive value that recalculates its value only when its dependencies change.
    let current_demo_name = Memo::new(move |_| params_demo_name());

    view! {
        <div class="hidden fixed top-14 z-30 md:flex md:sticky md:top-14 md:ml-2 w-[205px] h-[calc(100vh-3.5rem)] shrink-0">
            <aside data-name="__Sidenav_Docs" class="flex overflow-hidden flex-col flex-1 group/scrollbar-on-hover">
                <div class="flex overflow-hidden overflow-y-auto overscroll-y-contain flex-col gap-4 pb-4 w-full h-full rounded-[inherit] scrollbar__on_hover">
                    {all_arc_demos
                        .into_iter()
                        .map(|(title, demos_arc)| {
                            let base_path_clone = Arc::clone(&base_path);
                            let current_demo_name_clone = current_demo_name;
                            view! {
                                <div>
                                    <h4 class="my-1 text-sm font-semibold">{title}</h4>
                                    <ul class="ml-1 list-none">
                                        <For
                                            each=move || demos_arc.as_ref().clone()
                                            key=|demo| demo.path_url
                                            children=move |demo| {
                                                let demo_path = demo.path_url;
                                                let href = format!("{base_path_clone}/{demo_path}");
                                                let is_active = move || { demo_path == current_demo_name_clone() };
                                                let class = move || {
                                                    if is_active() {
                                                        "text-sm text-muted-foreground hover:underline font-bold"
                                                    } else {
                                                        "text-sm text-muted-foreground hover:underline"
                                                    }
                                                };

                                                view! {
                                                    <li>
                                                        <A href=href>
                                                            <span class=class>{demo.title}</span>
                                                        </A>
                                                    </li>
                                                }
                                            }
                                        />
                                    </ul>
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </aside>
        </div>
    }
}
