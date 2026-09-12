use dioxus::prelude::*;
use registry::hooks::use_resizable::{ResizablePreset, use_resizable};
use tw_merge::tw_merge;

#[derive(Clone, Copy)]
struct ResizableContext {
    container_element: Signal<Option<web_sys::Element>>,
    handle_element: Signal<Option<web_sys::Element>>,
    background_width: ReadSignal<f64>,
}

#[component]
pub fn Resizable(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] instance_id: Option<String>,
    #[props(optional)] preset: Option<ReadSignal<ResizablePreset>>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "border rounded-xl flex flex-row md:touch-none w-full",
        class.as_deref().unwrap_or("")
    );

    let container_element = use_signal(|| None::<web_sys::Element>);
    let handle_element = use_signal(|| None::<web_sys::Element>);
    let preset = preset.unwrap_or_else(|| ReadSignal::new(Signal::new(ResizablePreset::Desktop)));

    let resizable_state = use_resizable(container_element.into(), handle_element.into(), preset);

    use_context_provider(|| ResizableContext {
        container_element,
        handle_element,
        background_width: resizable_state.background_width,
    });

    rsx! {
        div {
            class: "{merged}",
            "data-resizable": instance_id.as_deref().unwrap_or(""),
            onmounted: move |event| {
                let mut container_element = container_element;
                container_element.set(event.data().downcast::<web_sys::Element>().cloned());
            },
            {children}
        }
    }
}

#[component]
pub fn ResizableContainer(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "flex items-center justify-center flex-[1_1_auto] min-w-[150px] bg-transparent h-full overflow-hidden",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        div { class: "{merged}", "data-resizable-container": true, {children} }
    }
}

#[component]
pub fn ResizableBackground(#[props(into, optional)] class: Option<String>) -> Element {
    let merged = tw_merge!(
        "flex-[0_0_auto] w-[0px] bg-muted transition-all duration-300",
        class.as_deref().unwrap_or("")
    );
    let ctx = try_consume_context::<ResizableContext>();
    let width = ctx.map(|c| c.background_width);

    rsx! {
        div {
            class: "{merged}",
            "data-resizable-bg": true,
            style: width.map_or_else(
                || "background-image: radial-gradient(circle, light-dark(#ccc, #444) 1px, transparent 1px); background-size: 20px 20px; background-attachment: fixed;".to_string(),
                |w| format!("width: {}px; background-image: radial-gradient(circle, light-dark(#ccc, #444) 1px, transparent 1px); background-size: 20px 20px; background-attachment: fixed;", w()),
            ),
        }
    }
}

#[component]
pub fn ResizableHandle(#[props(into, optional)] class: Option<String>) -> Element {
    let merged = tw_merge!(
        "hidden md:flex relative justify-center items-center p-0 w-3 -mr-2 translate-x-2 bg-transparent focus-visible:ring-1 focus-visible:ring-offset-1 focus-visible:outline-none after:inset-y-0 after:left-1/2 after:absolute after:right-0 after:top-1/2 after:h-8 after:w-[6px] after:-translate-y-1/2 after:translate-x-[-3px] after:rounded-full after:bg-neutral-200 dark:after:bg-neutral-600 after:transition-all after:hover:h-10 cursor-col-resize focus-visible:ring-ring transition-transform duration-200 select-none touch-none z-10",
        class.as_deref().unwrap_or("")
    );
    let ctx = try_consume_context::<ResizableContext>();

    rsx! {
        div {
            class: "{merged}",
            "data-resizable-handle": true,
            onmounted: move |event| {
                if let Some(mut ctx) = ctx {
                    ctx.handle_element.set(event.data().downcast::<web_sys::Element>().cloned());
                }
            },
        }
    }
}
