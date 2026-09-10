use dioxus::prelude::*;

use crate::domain::markdown_ui::components::my_resizable::{
    Resizable, ResizableBackground, ResizableContainer, ResizableHandle,
};

#[component]
pub fn ResizableWrapper(
    #[props(into, optional)] instance_id: Option<String>,
    #[props(into, optional)] preview_class: Option<String>,
    #[props(into, optional)] resizable_wrapper_class: Option<String>,
    children: Element,
) -> Element {
    let preview_classes =
        format!("flex justify-center items-center w-full min-h-[370px] {}", preview_class.as_deref().unwrap_or(""));

    rsx! {
        Resizable { instance_id: instance_id.clone(), class: resizable_wrapper_class.clone(),
            ResizableContainer {
                div {
                    "data-name": "Preview",
                    class: "{preview_classes}",
                    style: "--radius: 0.5rem;",
                    {children}
                }
            }
            ResizableHandle {}
            ResizableBackground {}
        }
    }
}
