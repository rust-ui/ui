use std::collections::HashSet;

use leptos::prelude::*;

/// A design parameter that can be locked to prevent randomization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LockableParam {
    Style,
    BaseColor,
    Theme,
    IconLibrary,
    Font,
    MenuAccent,
    MenuColor,
    Radius,
}

impl LockableParam {
    /// Display label for the param.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Style => "Style",
            Self::BaseColor => "Base Color",
            Self::Theme => "Theme",
            Self::IconLibrary => "Icon Library",
            Self::Font => "Font",
            Self::MenuAccent => "Menu Accent",
            Self::MenuColor => "Menu Color",
            Self::Radius => "Radius",
        }
    }

    /// All params in display order.
    pub const ALL: &'static [Self] = &[
        Self::Style,
        Self::BaseColor,
        Self::Theme,
        Self::IconLibrary,
        Self::Font,
        Self::MenuAccent,
        Self::MenuColor,
        Self::Radius,
    ];
}

/// Context that tracks which design params are locked against randomization.
///
/// Call `UseLocks::init()` once at the page root, then access via `use_locks()`
/// in any child component.
///
/// ```ignore
/// // In page component:
/// let _ = UseLocks::init();
///
/// // In child component:
/// let locks = use_locks();
/// let is_locked = locks.is_locked(LockableParam::Font);
///
/// view! {
///     <button on:click=move |_| locks.toggle_lock(LockableParam::Font)>
///         {move || if is_locked.get() { "Locked" } else { "Unlocked" }}
///     </button>
/// }
/// ```
#[derive(Clone, Copy)]
pub struct UseLocks {
    locks: RwSignal<HashSet<LockableParam>>,
}

impl UseLocks {
    /// Initialize and provide as context. No params are locked by default.
    #[must_use]
    pub fn init() -> Self {
        let hook = Self { locks: RwSignal::new(HashSet::new()) };
        provide_context(hook);
        hook
    }

    /// Returns a reactive signal that is `true` when `param` is locked.
    pub fn is_locked(&self, param: LockableParam) -> Signal<bool> {
        let locks = self.locks;
        Signal::derive(move || locks.with(|l| l.contains(&param)))
    }

    /// Toggle the lock state for `param`.
    pub fn toggle_lock(&self, param: LockableParam) {
        self.locks.update(|l| {
            if l.contains(&param) {
                l.remove(&param);
            } else {
                l.insert(param);
            }
        });
    }

    /// Lock a param explicitly.
    pub fn lock(&self, param: LockableParam) {
        self.locks.update(|l| {
            l.insert(param);
        });
    }

    /// Unlock a param explicitly.
    pub fn unlock(&self, param: LockableParam) {
        self.locks.update(|l| {
            l.remove(&param);
        });
    }

    /// Returns all currently locked params. Tracked when called inside a reactive closure.
    pub fn locked_params(&self) -> HashSet<LockableParam> {
        self.locks.get()
    }

    /// `true` when `param` is NOT locked (safe to randomize).
    pub fn can_randomize(&self, param: LockableParam) -> Signal<bool> {
        let locks = self.locks;
        Signal::derive(move || !locks.with(|l| l.contains(&param)))
    }
}

/// Access the `UseLocks` context initialized by `UseLocks::init()`.
pub fn use_locks() -> UseLocks {
    expect_context::<UseLocks>()
}
