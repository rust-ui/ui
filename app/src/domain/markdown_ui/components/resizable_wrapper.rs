use leptos::prelude::*;

use crate::domain::markdown_ui::components::my_resizable::{
    Resizable, ResizableBackground, ResizableContainer, ResizableHandle,
};

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn ResizableWrapper(children: Children, #[prop(into, optional)] preview_class: String) -> impl IntoView {
    let preview_classes = format!("flex justify-center items-center w-full min-h-[370px] {}", preview_class);

    view! {
        <Resizable>
            <ResizableContainer>
                <div data-name="Preview" class=preview_classes style="--radius: 0.5rem;">
                    {children()}
                </div>
            </ResizableContainer>

            <ResizableHandle />
            <ResizableBackground attr:style="background-image: radial-gradient(circle, light-dark(#ccc, #444) 1px, transparent 1px); background-size: 20px 20px; background-attachment: fixed;" />
        </Resizable>
    }
}
