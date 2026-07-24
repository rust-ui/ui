use icons::Check;
use leptos::prelude::*;
use leptos_ui::{clx, variants, void};
use tw_merge::tw_merge;

use crate::hooks::use_stepper::{StepState, StepperContext, use_stepper};

/* ========================================================== */
/*                       Enums                                */
/* ========================================================== */

/// Layout direction for a `Stepper` — controls both the root flex direction
/// and which `StepperSeparator` styling (inline bar vs. absolute vertical
/// line) applies, via the `data-orientation` attribute on the root element.
#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum StepperOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy)]
struct StepperItemCtx {
    step: usize,
    state: Memo<StepState>,
}

/* ========================================================== */
/*                  Tailwind Variants                         */
/* ========================================================== */

variants! {
    StepperIndicator {
        base: "flex size-8 shrink-0 items-center justify-center rounded-full border text-sm font-medium transition-colors",
        variants: {
            variant: {
                Pending: "border-border bg-background text-muted-foreground",
                Active: "border-primary bg-primary text-primary-foreground",
                Completed: "border-primary bg-primary text-primary-foreground",
                Disabled: "border-border bg-muted text-muted-foreground/50",
            }
        }
    }
}

impl From<StepState> for StepperIndicatorVariant {
    fn from(state: StepState) -> Self {
        match state {
            StepState::Completed => StepperIndicatorVariant::Completed,
            StepState::Active => StepperIndicatorVariant::Active,
            StepState::Pending => StepperIndicatorVariant::Pending,
            StepState::Disabled => StepperIndicatorVariant::Disabled,
        }
    }
}

/* ========================================================== */
/*            Structural components (clx! / void!)            */
/* ========================================================== */

mod components {
    use super::*;

    clx! {
        StepperTitle, div,
        "text-sm font-medium text-foreground transition-colors",
        "group-data-[state=Pending]/stepper-item:text-muted-foreground",
        "group-data-[state=Disabled]/stepper-item:text-muted-foreground/50"
    }

    clx! {
        StepperDescription, div,
        "text-sm text-muted-foreground transition-colors",
        "group-data-[state=Disabled]/stepper-item:text-muted-foreground/50"
    }

    void! {
        StepperSeparator, div,
        "shrink-0 bg-border transition-colors",
        "group-data-[orientation=Horizontal]/stepper:self-center group-data-[orientation=Horizontal]/stepper:h-0.5 group-data-[orientation=Horizontal]/stepper:w-full",
        "group-data-[orientation=Vertical]/stepper:absolute group-data-[orientation=Vertical]/stepper:top-8 group-data-[orientation=Vertical]/stepper:left-4 group-data-[orientation=Vertical]/stepper:h-full group-data-[orientation=Vertical]/stepper:w-0.5",
        "group-data-[state=Completed]/stepper-item:bg-primary"
    }
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Root provider — builds the shared `StepperContext` and exposes it to
/// every descendant `StepperItem`/`StepperTrigger` via `provide_context`.
#[component]
pub fn Stepper(
    total_steps: usize,
    #[prop(default = 0)] default_step: usize,
    #[prop(default = StepperOrientation::Horizontal)] orientation: StepperOrientation,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let ctx = use_stepper(total_steps, default_step);
    provide_context(ctx);

    let orientation_str = orientation.to_string();
    let class = tw_merge!(
        "group/stepper flex w-full",
        if orientation == StepperOrientation::Horizontal { "flex-row items-start" } else { "flex-col" },
        class
    );

    view! {
        <div class=class data-name="Stepper" data-orientation=orientation_str>
            {children()}
        </div>
    }
}

/// One step's wrapper — reads the shared context to derive this step's
/// `StepState`, folding in the locally-supplied `disabled` prop.
#[component]
pub fn StepperItem(
    step: usize,
    #[prop(default = false)] disabled: bool,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<StepperContext>();

    let state = Memo::new(move |_| if disabled { StepState::Disabled } else { ctx.step_state.run(step) });
    provide_context(StepperItemCtx { step, state });

    let class = tw_merge!(
        "group/stepper-item relative flex flex-1 items-start gap-2",
        "group-data-[orientation=Vertical]/stepper:flex-col",
        class
    );

    view! {
        <div class=class data-name="StepperItem" data-state=move || state.get().to_string()>
            {children()}
        </div>
    }
}

/// Clickable native `<button>` for a step — navigates via `stepper_ctx.go_to`,
/// blocks interaction when the step is disabled, and marks itself
/// `aria-current="step"` when active.
#[component]
pub fn StepperTrigger(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let stepper_ctx = expect_context::<StepperContext>();
    let item_ctx = expect_context::<StepperItemCtx>();
    let step = item_ctx.step;

    let is_disabled = Memo::new(move |_| item_ctx.state.get() == StepState::Disabled);
    let is_active = Memo::new(move |_| item_ctx.state.get() == StepState::Active);

    let class = tw_merge!(
        "group/stepper-trigger flex flex-1 items-center gap-2 rounded-md text-left outline-none cursor-pointer",
        "focus-visible:ring-[3px] focus-visible:ring-ring/50",
        "disabled:pointer-events-none disabled:opacity-50 disabled:cursor-not-allowed",
        "group-data-[orientation=Vertical]/stepper:w-full",
        class
    );

    view! {
        <button
            type="button"
            class=class
            data-name="StepperTrigger"
            disabled=move || is_disabled.get()
            aria-current=move || if is_active.get() { "step" } else { "" }
            on:click=move |_| stepper_ctx.go_to.run(step)
        >
            {children()}
        </button>
    }
}

/// Step's visual dot — number by default, checkmark once completed, or fully
/// custom content via `children`. Colors come from `StepperIndicatorVariant`,
/// derived from this step's `StepState`.
#[component]
pub fn StepperIndicator(
    #[prop(into, optional)] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let item_ctx = expect_context::<StepperItemCtx>();

    let indicator_class = move || {
        let variant: StepperIndicatorVariant = item_ctx.state.get().into();
        StepperIndicatorClass { variant }.with_class(class.clone())
    };

    view! {
        <span class=indicator_class data-name="StepperIndicator" aria-hidden="true">
            {match children {
                Some(children) => children().into_any(),
                None => {
                    view! {
                        {move || {
                            if item_ctx.state.get() == StepState::Completed {
                                view! { <Check class="size-4" /> }.into_any()
                            } else {
                                view! { {(item_ctx.step + 1).to_string()} }.into_any()
                            }
                        }}
                    }
                        .into_any()
                }
            }}
        </span>
    }
}
