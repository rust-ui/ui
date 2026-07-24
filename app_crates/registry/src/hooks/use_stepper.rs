use leptos::prelude::*;

/// Visual/interactive state of a single step, relative to the current index.
///
/// `Completed`/`Active`/`Pending` are derived automatically from index
/// comparison. `Disabled` is not produced by this hook — it's applied by
/// the caller (e.g. `StepperItem`) on top of the computed state, since it
/// depends on external conditions the hook has no visibility into.
#[derive(Clone, Copy, PartialEq, Eq, strum::Display)]
pub enum StepState {
    Completed,
    Active,
    Pending,
    Disabled,
}

/// Shared reactive state for a `Stepper` instance, provided via
/// `provide_context` and consumed by `StepperItem`/`StepperTrigger`.
#[derive(Clone)]
pub struct StepperContext {
    pub current_index: RwSignal<usize>,
    pub total_steps: usize,
    pub can_go_prev: Signal<bool>,
    pub can_go_next: Signal<bool>,
    pub go_next: Callback<(), ()>,
    pub go_prev: Callback<(), ()>,
    pub go_to: Callback<usize, ()>,
    pub step_state: Callback<usize, StepState>,
}

/// Builds the controlled navigation state for a stepper with `total_steps`
/// steps, starting at `default_index`.
///
/// All navigation methods (`go_next`, `go_prev`, `go_to`) clamp to
/// `[0, total_steps)`, so `current_index` can never be set out of range —
/// callers indexing a step list with it don't need to re-validate.
pub fn use_stepper(total_steps: usize, default_index: usize) -> StepperContext {
    // Clamp in case `default_index` is out of range (e.g. caller passes total_steps itself).
    let current_index = RwSignal::new(default_index.min(total_steps.saturating_sub(1)));

    // Reactive rather than one-shot, so nav buttons can bind `disabled` directly
    // and re-evaluate whenever `current_index` changes.
    let can_go_prev = Signal::derive(move || current_index.get() > 0);
    let can_go_next = Signal::derive(move || current_index.get() + 1 < total_steps);

    let go_prev = Callback::new(move |_| {
        if current_index.get() > 0 {
            current_index.update(|i| *i -= 1);
        }
    });

    let go_next = Callback::new(move |_| {
        if current_index.get() + 1 < total_steps {
            current_index.update(|i| *i += 1);
        }
    });

    // Backs clickable step triggers — jumps straight to an arbitrary index
    // rather than stepping by one, so out-of-range values need their own guard.
    let go_to = Callback::new(move |index: usize| {
        if index < total_steps {
            current_index.set(index);
        }
    });

    // Maps the issue's three-way rule (step < current -> completed, == -> active,
    // > -> pending) onto Ordering so it reads as one exhaustive match.
    let step_state = Callback::new(move |step: usize| {
        let current = current_index.get();

        match step.cmp(&current) {
            std::cmp::Ordering::Less => StepState::Completed,
            std::cmp::Ordering::Equal => StepState::Active,
            std::cmp::Ordering::Greater => StepState::Pending,
        }
    });

    StepperContext { current_index, total_steps, can_go_prev, can_go_next, go_next, go_prev, go_to, step_state }
}
