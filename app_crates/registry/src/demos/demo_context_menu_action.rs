use dioxus::prelude::*;
use icons::{Check, Copy, Settings2};

use crate::hooks::use_press_hold::use_press_hold;
use crate::ui::context_menu::{
    ContextMenu, ContextMenuAction, ContextMenuContent, ContextMenuGroup, ContextMenuItem, ContextMenuLabel,
    ContextMenuTrigger, close_context_menu,
};
use crate::ui::separator::Separator;

#[component]
pub fn DemoContextMenuAction() -> Element {
    let on_confirm = Callback::new(move |_: ()| {
        close_context_menu();
    });

    rsx! {
        ContextMenu {
            ContextMenuTrigger {
                class: "flex justify-center items-center text-sm rounded-md border transition-colors h-[150px] w-[300px] bg-card hover:bg-muted/50",
                "Right click here"
            }

            ContextMenuContent {
                ContextMenuLabel { "Actions" }

                ContextMenuGroup {
                    ContextMenuItem {
                        ContextMenuAction {
                            Copy {}
                            "Copy"
                        }
                    }
                    ContextMenuItem {
                        ContextMenuAction {
                            Settings2 {}
                            "Settings"
                        }
                    }
                }

                Separator { class: "my-1" }

                ContextMenuItem { class: "p-0",
                    PressHoldAction { on_complete: on_confirm }
                }
            }
        }
    }
}

#[component]
fn PressHoldAction(on_complete: Callback<()>) -> Element {
    let press_hold = use_press_hold(1500, on_complete, false);

    let ph1 = press_hold.clone();
    let ph2 = press_hold.clone();
    let ph3 = press_hold.clone();
    let ph4 = press_hold.clone();

    let progress_style = move || {
        let width_percent = (press_hold.progress_signal)() * 100.0;
        format!(
            "position: absolute; left: 0; top: 0; bottom: 0; width: {width_percent:.1}%; background: rgba(34, 197, 94, 0.3); pointer-events: none; border-radius: inherit;"
        )
    };

    rsx! {
        button {
            class: "flex relative gap-2 items-center py-1.5 px-2 w-full text-sm rounded-sm transition-colors select-none text-primary hover:bg-primary/10",
            onpointerdown: move |_| ph1.on_pointer_down(),
            onpointerup: move |_| ph2.on_pointer_up(),
            onpointerleave: move |_| ph3.on_pointer_up(),
            onpointercancel: move |_| ph4.on_pointer_up(),
            span { style: "{progress_style()}" }
            span { class: "flex relative z-10 gap-2 items-center",
                Check { class: "size-4" }
                "Hold to Confirm"
            }
        }
    }
}
