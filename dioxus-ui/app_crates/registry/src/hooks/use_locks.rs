use std::collections::HashSet;

use dioxus::prelude::*;

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
#[derive(Clone, Copy)]
pub struct UseLocks {
    locks: Signal<HashSet<LockableParam>>,
}

impl UseLocks {
    /// Initialize and provide as context. No params are locked by default.
    #[must_use]
    pub fn init() -> Self {
        let hook = Self { locks: use_signal(HashSet::new) };
        provide_context(hook);
        hook
    }

    /// Returns whether `param` is currently locked (not reactive — call inside a closure).
    pub fn is_locked(&self, param: LockableParam) -> bool {
        self.locks.read().contains(&param)
    }

    /// Toggle the lock state for `param`.
    pub fn toggle_lock(&mut self, param: LockableParam) {
        self.locks.with_mut(|l| {
            if l.contains(&param) {
                l.remove(&param);
            } else {
                l.insert(param);
            }
        });
    }

    /// Lock a param explicitly.
    pub fn lock(&mut self, param: LockableParam) {
        self.locks.with_mut(|l| {
            l.insert(param);
        });
    }

    /// Unlock a param explicitly.
    pub fn unlock(&mut self, param: LockableParam) {
        self.locks.with_mut(|l| {
            l.remove(&param);
        });
    }

    /// Returns all currently locked params.
    pub fn locked_params(&self) -> HashSet<LockableParam> {
        self.locks.read().clone()
    }

    /// `true` when `param` is NOT locked (safe to randomize).
    pub fn can_randomize(&self, param: LockableParam) -> bool {
        !self.locks.read().contains(&param)
    }
}

/// Access the `UseLocks` context initialized by `UseLocks::init()`.
pub fn use_locks() -> UseLocks {
    use_context::<UseLocks>()
}
