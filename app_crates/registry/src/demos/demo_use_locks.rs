use dioxus::prelude::*;
use icons::{Lock, LockOpen};

use crate::hooks::use_locks::{LockableParam, UseLocks, use_locks};
use crate::ui::button::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn DemoUseLocks() -> Element {
    let _locks = UseLocks::init();

    rsx! { DemoUseLocksInner {} }
}

#[component]
fn DemoUseLocksInner() -> Element {
    let locks = use_locks();

    rsx! {
        div { class: "flex flex-col gap-4 my-4 w-full max-w-sm",
            for param in LockableParam::ALL.iter().copied() {
                {
                    let mut locks_for_click = locks;
                    rsx! {
                        div { class: "flex gap-3 justify-between items-center py-2 px-3 rounded-md border",
                            span {
                                class: if locks.is_locked(param) {
                                    "text-sm text-muted-foreground line-through"
                                } else {
                                    "text-sm text-foreground"
                                },
                                "{param.label()}"
                            }
                            Button {
                                variant: if locks.is_locked(param) {
                                    ButtonVariant::Default
                                } else {
                                    ButtonVariant::Ghost
                                },
                                size: ButtonSize::Icon,
                                onclick: move |_| locks_for_click.toggle_lock(param),
                                if locks.is_locked(param) {
                                    Lock { class: "size-3.5" }
                                } else {
                                    LockOpen { class: "size-3.5" }
                                }
                            }
                        }
                    }
                }
            }

            p { class: "pt-1 text-xs text-center text-muted-foreground",
                {
                    let count = locks.locked_params().len();
                    if count == 0 {
                        "No params locked - all will be randomized".to_string()
                    } else {
                        format!("{count} param{} locked", if count == 1 { "" } else { "s" })
                    }
                }
            }
        }
    }
}
