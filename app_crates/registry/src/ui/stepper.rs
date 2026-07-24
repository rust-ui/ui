use leptos::prelude::*;
use tw_merge::tw_merge;

use crate::hooks::use_stepper::{StepState, use_stepper};

/* ========================================================== */
/*                       Enums                                */
/* ========================================================== */

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum StepperOrientation {
    #[default]
    Horizontal,
    Vertical,
}

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
    let ctx = expect_context::<crate::hooks::use_stepper::StepperContext>();

    let state = Memo::new(move |_| if disabled { StepState::Disabled } else { ctx.step_state.run(step) });

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
