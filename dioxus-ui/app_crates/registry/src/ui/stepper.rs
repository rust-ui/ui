use dioxus::prelude::*;
use icons::Check;
use tw_merge::tw_merge;

use crate::hooks::use_stepper::{StepState, StepperContext, use_stepper};

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum StepperOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl StepperOrientation {
    fn as_str(&self) -> &'static str {
        match self {
            StepperOrientation::Horizontal => "Horizontal",
            StepperOrientation::Vertical => "Vertical",
        }
    }
}

#[derive(Clone, Copy)]
struct StepperItemCtx {
    step: usize,
    disabled: bool,
}

#[component]
pub fn StepperTitle(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "text-sm font-medium text-foreground transition-colors group-data-[state=Pending]/stepper-item:text-muted-foreground group-data-[state=Disabled]/stepper-item:text-muted-foreground/50",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { "data-name": "StepperTitle", class: "{merged}", {children} } }
}

#[component]
pub fn StepperDescription(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!(
        "text-sm text-muted-foreground transition-colors group-data-[state=Disabled]/stepper-item:text-muted-foreground/50",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { "data-name": "StepperDescription", class: "{merged}", {children} } }
}

#[component]
pub fn StepperSeparator(#[props(into, optional)] class: Option<String>) -> Element {
    let merged = tw_merge!(
        "absolute bg-border transition-colors",
        "group-data-[orientation=Horizontal]/stepper:top-4 group-data-[orientation=Horizontal]/stepper:left-[calc(50%+1.5rem)] group-data-[orientation=Horizontal]/stepper:w-[calc(100%-3rem)] group-data-[orientation=Horizontal]/stepper:h-0.5 group-data-[orientation=Horizontal]/stepper:-translate-y-1/2",
        "group-data-[orientation=Vertical]/stepper:top-8 group-data-[orientation=Vertical]/stepper:left-4 group-data-[orientation=Vertical]/stepper:h-full group-data-[orientation=Vertical]/stepper:w-0.5",
        "group-data-[state=Completed]/stepper-item:bg-primary",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { "data-name": "StepperSeparator", class: "{merged}" } }
}

#[component]
pub fn Stepper(
    total_steps: usize,
    #[props(default = 0)] default_step: usize,
    #[props(default = StepperOrientation::Horizontal)] orientation: StepperOrientation,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let ctx = use_stepper(total_steps, default_step);
    use_context_provider(|| ctx);

    let merged = tw_merge!(
        "group/stepper flex w-full",
        if orientation == StepperOrientation::Horizontal { "flex-row flex-wrap items-start" } else { "flex-col" },
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div { "data-name": "Stepper", "data-orientation": orientation.as_str(), class: "{merged}", {children} }
    }
}

#[component]
pub fn StepperItem(
    step: usize,
    #[props(default = false)] disabled: bool,
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let stepper_ctx = use_context::<StepperContext>();
    use_context_provider(|| StepperItemCtx { step, disabled });

    let state = if disabled { StepState::Disabled } else { stepper_ctx.step_state(step) };
    let merged = tw_merge!(
        "group/stepper-item relative flex flex-1 flex-col items-center gap-2",
        "group-data-[orientation=Vertical]/stepper:flex-row group-data-[orientation=Vertical]/stepper:items-start",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div { "data-name": "StepperItem", "data-state": state.as_str(), class: "{merged}", {children} }
    }
}

#[component]
pub fn StepperTrigger(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let stepper_ctx = use_context::<StepperContext>();
    let item_ctx = use_context::<StepperItemCtx>();
    let step = item_ctx.step;

    let state = if item_ctx.disabled { StepState::Disabled } else { stepper_ctx.step_state(step) };
    let is_disabled = state == StepState::Disabled;
    let is_active = state == StepState::Active;

    let merged = tw_merge!(
        "group/stepper-trigger flex flex-1 flex-col items-center gap-2 rounded-md text-center outline-none cursor-pointer",
        "focus-visible:ring-[3px] focus-visible:ring-ring/50",
        "disabled:pointer-events-none disabled:opacity-50 disabled:cursor-not-allowed",
        "group-data-[orientation=Vertical]/stepper:flex-row group-data-[orientation=Vertical]/stepper:items-start group-data-[orientation=Vertical]/stepper:text-left group-data-[orientation=Vertical]/stepper:w-full",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        button {
            r#type: "button",
            "data-name": "StepperTrigger",
            disabled: is_disabled,
            "aria-current": if is_active { "step" } else { "" },
            class: "{merged}",
            onclick: move |_| stepper_ctx.go_to(step),
            {children}
        }
    }
}

#[component]
pub fn StepperIndicator(#[props(into, optional)] class: Option<String>, children: Option<Element>) -> Element {
    let stepper_ctx = use_context::<StepperContext>();
    let item_ctx = use_context::<StepperItemCtx>();
    let state = if item_ctx.disabled { StepState::Disabled } else { stepper_ctx.step_state(item_ctx.step) };

    let variant_class = match state {
        StepState::Pending => "border-border bg-background text-muted-foreground",
        StepState::Active => "border-primary bg-primary text-primary-foreground",
        StepState::Completed => "border-primary bg-primary text-primary-foreground",
        StepState::Disabled => "border-border bg-muted text-muted-foreground/50",
    };
    let merged = tw_merge!(
        "relative z-10 flex size-8 shrink-0 items-center justify-center rounded-full border text-sm font-medium transition-colors",
        variant_class,
        class.as_deref().unwrap_or("")
    );

    rsx! {
        span { "data-name": "StepperIndicator", "aria-hidden": "true", class: "{merged}",
            match children {
                Some(children) => children,
                None => {
                    if state == StepState::Completed {
                        rsx! { Check { class: "size-4" } }
                    } else {
                        rsx! { "{item_ctx.step + 1}" }
                    }
                }
            }
        }
    }
}
