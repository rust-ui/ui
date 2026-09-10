use dioxus::prelude::*;

/// Visual/interactive state of a single step, relative to the current index.
///
/// `Completed`/`Active`/`Pending` are derived automatically from index
/// comparison. `Disabled` is not produced by this hook — it's applied by
/// the caller (e.g. `StepperItem`) on top of the computed state, since it
/// depends on external conditions the hook has no visibility into.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StepState {
    Completed,
    Active,
    Pending,
    Disabled,
}

impl StepState {
    pub fn as_str(&self) -> &'static str {
        match self {
            StepState::Completed => "Completed",
            StepState::Active => "Active",
            StepState::Pending => "Pending",
            StepState::Disabled => "Disabled",
        }
    }
}

/// Shared reactive state for a `Stepper` instance, provided via
/// `provide_context` and consumed by `StepperItem`/`StepperTrigger`.
///
/// All navigation methods (`go_next`, `go_prev`, `go_to`) clamp to
/// `[0, total_steps)`, so `current_index` can never be set out of range —
/// callers indexing a step list with it don't need to re-validate.
#[derive(Clone, Copy)]
pub struct StepperContext {
    pub current_index: Signal<usize>,
    pub total_steps: usize,
}

impl StepperContext {
    /// `true` when there is a previous state to undo to.
    pub fn can_go_prev(&self) -> bool {
        (self.current_index)() > 0
    }

    /// `true` when there is a future state to redo to.
    pub fn can_go_next(&self) -> bool {
        (self.current_index)() + 1 < self.total_steps
    }

    pub fn go_prev(&self) {
        let mut index = self.current_index;
        if index() > 0 {
            index.with_mut(|i| *i -= 1);
        }
    }

    pub fn go_next(&self) {
        let mut index = self.current_index;
        if index() + 1 < self.total_steps {
            index.with_mut(|i| *i += 1);
        }
    }

    /// Backs clickable step triggers — jumps straight to an arbitrary index
    /// rather than stepping by one, so out-of-range values need their own guard.
    pub fn go_to(&self, step: usize) {
        if step < self.total_steps {
            let mut index = self.current_index;
            index.set(step);
        }
    }

    /// Maps the issue's three-way rule (step < current -> completed, == -> active,
    /// > -> pending) onto Ordering so it reads as one exhaustive match.
    pub fn step_state(&self, step: usize) -> StepState {
        let current = (self.current_index)();
        match step.cmp(&current) {
            std::cmp::Ordering::Less => StepState::Completed,
            std::cmp::Ordering::Equal => StepState::Active,
            std::cmp::Ordering::Greater => StepState::Pending,
        }
    }
}

/// Builds the controlled navigation state for a stepper with `total_steps`
/// steps, starting at `default_index`.
pub fn use_stepper(total_steps: usize, default_index: usize) -> StepperContext {
    // Clamp in case `default_index` is out of range (e.g. caller passes total_steps itself).
    let current_index = use_signal(|| default_index.min(total_steps.saturating_sub(1)));
    StepperContext { current_index, total_steps }
}
